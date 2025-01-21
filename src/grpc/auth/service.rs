use serde_json::json;
use uuid::Uuid;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use chrono::{Utc, Duration};
use diesel::{PgConnection};
use tonic::{Request, Response, Status};
use serde::{ Serialize, Deserialize };

use jsonwebtoken::{encode, Header, decode, Validation, DecodingKey, EncodingKey};

use crate::utils;
use crate::models;

use ipnet::IpNet; // Import ipnet types
use std::net::IpAddr;

use super::v1::{AccessToken, AccessTokenTokenRequest, LoginResponse, RefreshToken, Token, TokenAlgorithm, UsernameLoginRequest};
use super::v1::auth_server::Auth;
use crate::grpc::users::v1::UserResponse;

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessTokenClaims {
    iss: String,
    sub: String,
    iat: i64,
    exp: i64,
    jti: Uuid
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshTokenClaims {
    iss: String,
    sub: String,
    iat: i64,
    exp: i64,
    sid: Uuid,
}


async fn signin_at(user: models::User, jti: Uuid) -> Option<AccessToken> {
    let now = Utc::now();
    let iat = now.timestamp();
    let exp = (now + Duration::weeks(1)).timestamp();

    let claims = AccessTokenClaims { 
        iss: "example-issuer".to_string(),
        sub: user.user_uuid.to_string(),
        iat,
        exp,
        jti
    };

    let value = encode(&Header::default(), &claims, &EncodingKey::from_secret("hello world".as_bytes())).unwrap();

    Some(AccessToken {
        token: Option::from(Token {
            issued_at: now.timestamp(),
            algorithm: TokenAlgorithm::AlgorithmUnspecified as i32,
            expires_at: exp,
            value: value,
            scope: "".to_string(),
            metadata: HashMap::new(),
        }),
        jwt_id: jti.to_string(),
        audience: "api-service".to_string(),
        issuer: "example-issuer".to_string(),
        roles: vec![],
        user_id: user.user_uuid.to_string()
    })
}


async fn signin_rt(user: models::User, sid: Uuid) -> Option<RefreshToken> {
    let now = Utc::now();
    let iat = now.timestamp();
    let exp = (now + Duration::days(30)).timestamp();

    let claims = RefreshTokenClaims { 
        iss: "example-issuer".to_string(),
        sub: user.user_uuid.to_string(),
        iat,
        exp,
        sid
    };

    let value = encode(&Header::default(), &claims, &EncodingKey::from_secret("hello world".as_bytes())).unwrap();

    Some(RefreshToken {
        token: Option::from(Token {
            issued_at: now.timestamp(),
            algorithm: TokenAlgorithm::AlgorithmUnspecified as i32,
            expires_at: exp,
            value: value,
            scope: "".to_string(),
            metadata: HashMap::new(),
        }),
        issuer: "example-issuer".to_string(),
        user_id: user.user_uuid.to_string(),
        session_id: sid.to_string()
    })
}

pub struct AuthService {
    database: Arc<Mutex<PgConnection>>
}

impl AuthService {
    pub fn new(database: Arc<Mutex<PgConnection>>) -> Self {
        Self {
            database,
        }
    }
}

#[tonic::async_trait]
impl Auth for AuthService {
    async fn username_login(
        &self,
        request: Request<UsernameLoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {

        let inputs = request.into_inner();
        
        let user = {
            let mut database = self.database.lock().unwrap();
            models::User::find_by_username(&mut database, inputs.username).map_err(|e| Status::internal(format!("Error finding user: {}", e)))?
        };

        let valid = utils::verify_password(&inputs.password, &user.password_hash);

        let valid = match valid {
            Ok(valid) => valid,
            Err(e) => {
                // make failed login attempt here with device information
                return Err(Status::internal(format!("Error verifying password: {}", e)))
            },
        };

        if !valid {
            return Err(Status::unauthenticated("Invalid password"));
        }

        let device_uuid =  Uuid::parse_str(&inputs.device_id).map_err(|e| Status::invalid_argument(format!("Invalid device ID: {}", e)))?;

        // let device = {
        //     let mut database = self.database.lock().unwrap();
        //     models::Device::find_by_uuid(&mut database, device_uuid).map_err(|e| Status::internal(format!("Error finding device: {}", e)))?
        // };

        let ip_addr = Some(inputs.ip.parse::<IpAddr>().unwrap()); // Example IP address

        // Convert the IP address to a network (e.g., /24 for IPv4 or /64 for IPv6)
        let ip_address: Option<IpNet> = ip_addr.map(|ip| {
            match ip {
                // Create a /24 network for IPv4 (common for local networks)
                IpAddr::V4(ipv4) => IpNet::V4(ipv4.into()),
                // Create a /64 network for IPv6 (common for local networks)
                IpAddr::V6(ipv6) => IpNet::V6(ipv6.into())
            }
        });

        let now = Utc::now();
        let exp = now + Duration::days(30);
        
        let session = {
            let mut database = self.database.lock().unwrap();
        
            let new_session = models::NewSession{
                device_uuid: device_uuid,
                user_uuid: user.user_uuid,
                ip_address: ip_address.unwrap(),
                metadata: json!({}),
                expires_at: exp,
            };

            models::Session::create(&mut database, new_session).map_err(|e| Status::internal(format!("Error creating Session: {}", e)))?
        };


        let jti = Uuid::new_v4();

        /*
            - Get Device ✅
            - Create Session ✅
            - Assign JWT
            - Authorization
         */

        /*

            - login attempts 
            1. Device
            2. Store Session
        
        */

        Ok(Response::new(LoginResponse {
            access_token: signin_at(user.clone(), jti).await,
            refresh_token: signin_rt(user.clone(), session.session_uuid).await,
            message: "".to_string(),
            success: true
        }))
    }

    async fn get_user_token(
        &self,
        request: Request<AccessTokenTokenRequest>,
    ) -> Result<Response<UserResponse>, Status> {
        let inputs = request.into_inner();

        let claims = decode::<AccessTokenClaims>(
            &inputs.value,
            &DecodingKey::from_secret("hello world".as_bytes()),
            &Validation::default(),
        ).map_err(|e| Status::invalid_argument(format!("Invalid token: {}", e)))?;

        let user_uuid = Uuid::parse_str(&claims.claims.sub).map_err(|e| Status::invalid_argument(format!("Invalid user UUID: {}", e))).unwrap();

        let user = {
            let mut database = self.database.lock().unwrap();
            models::User::find_user_uuid(&mut database, user_uuid)
                .map_err(|e| Status::internal(format!("Error finding user: {}", e)))?
        };

        Ok(Response::new(UserResponse {
            id: user.user_uuid.to_string(),
            username: user.username,
            status: user.status as i32,
            onboarded: true,
            is_verified: true,
        }))
    }
}