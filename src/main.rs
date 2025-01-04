mod models;
mod schema;

use std::borrow::Borrow;
use std::env;
use argon2::{Argon2, PasswordHasher, password_hash::Salt};
use rand::Rng;
use dotenvy::dotenv;
use serde_json::json;
use regex::Regex;

use std::sync::{Arc, Mutex};
use diesel::{PgConnection, Connection};


pub mod users_v1 {
    tonic::include_proto!("ingot.api.users.v1"); // The string specified here must match the proto package name
}

use tonic::{transport::Server, Request, Response, Status};

use users_v1::users_server::{Users, UsersServer};
use users_v1::{CreateUserRequest, User, UserStatus};

// #[derive(Default)]
pub struct UsersService {
    database: Arc<Mutex<PgConnection>>,
}

impl UsersService {
    pub fn new(database: PgConnection) -> Self {
        Self {
            database: Arc::new(Mutex::new(database))
        }
    }
}

#[tonic::async_trait]
impl Users for UsersService {
    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<User>, Status> {
        let user = request.into_inner();
        let password = user.password.borrow();

        let username_regex = Regex::new(r"^[a-zA-Z0-9_]+$").unwrap();

        if !username_regex.is_match(&user.username) {
            return Err(Status::invalid_argument("Username must contain only letters, numbers, and underscores"));
        }

        if user.username.len() < 3 {
            return Err(Status::invalid_argument("Username must be at least 3 characters"));
        }

        if user.username.len() > 32 {
            return Err(Status::invalid_argument("Username must be less than 32 characters"));
        }

        if user.password.len() < 8 {
            return Err(Status::invalid_argument("Password must be at least 8 characters"));
        }

        if user.password.len() > 64 {
            return Err(Status::invalid_argument("Password must be less than 64 characters"));
        }

        let has_uppercase = Regex::new(r"[A-Z]").unwrap().is_match(password);

        if !has_uppercase {
            return Err(Status::invalid_argument("Password must contain at least one uppercase letter"));
        }

        let has_lowercase = Regex::new(r"[a-z]").unwrap().is_match(password);

        if !has_lowercase {
            return Err(Status::invalid_argument("Password must contain at least one lowercase letter"));
        }

        let has_number = Regex::new(r"[0-9]").unwrap().is_match(password);

        if !has_number {
            return Err(Status::invalid_argument("Password must contain at least one number"));
        }

        let has_special = Regex::new(r"[!@#$%^&*(),.?:{}|<>]").unwrap().is_match(password);

        if !has_special {
            return Err(Status::invalid_argument("Password must contain at least one special character"));
        }

        // if user.password != user.confirm_password {
        //     return Err(Status::invalid_argument("Passwords do not match"));
        // }

        // if user.email.is_empty() {
        //     return Err(Status::invalid_argument("Email is required"));
        // }

        let salt_str = "YmFkIHNhbHQh";
        let salt: Salt = salt_str.try_into().unwrap();
        let hashed_password = hash_password(&user.password, salt).unwrap();

        let user = models::NewUser {
            username: user.username.to_lowercase(),
            password_hash: hashed_password,
            metadata: json!({}),
        };

        let database = self.database.lock();
        let user = models::User::create(&mut database.unwrap(), user)
            .map_err(|e| Status::internal(format!("Error creating user: {}", e)))?;

        Ok(Response::new(User {
            id: user.user_uuid.to_string(),
            username: user.username,
            status: user.status as i32,
            is_verified: user.is_verified,
            onboarded: user.onboarded,
        }))

        // timezone: user.timezone,
        // locale: user.locale,
        // is_active: user.is_active,
        // is_verified: user.is_verified,
        // created_at: user.created_at,
        // updated_at: user.updated_at,
    }
}


fn hash_password(password: &str, salt: Salt) -> Result<Vec<u8>, argon2::password_hash::Error> {
    // Set up the Argon2 configuration
    // let config = Config::default();
    let argon2 = Argon2::default();

    // Hash the password
    let hash = argon2.hash_password(password.as_bytes(), salt)?;

    // Convert the hash to bytes
    Ok(hash.hash.unwrap().as_bytes().to_vec())
}


/*

MVP
- Create User
- Verify Email
- Verify Phone
- Login
- Refresh Token
- Logout
- Change Password
- Forgot Password
- Reset Password
- Update User
- Delete User
- Get User
- Get Session
- Get User Roles
- Get User Permissions
- Get User Actions
- Get User Organizations
- Get User Devices
- Get User Emails

*/

pub fn connect_db() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let database = connect_db();

    println!("Starting server...");

    let addr = "[::1]:50051".parse()?;
    let users_service = UsersService::new(database);

    Server::builder()
        .add_service(UsersServer::new(users_service))
        .serve(addr)
        .await?;

    Ok(())
}

/*

users:  
    - user_id
    - username (unique, indexed)
    - password_hash
    - timezone
    - locale
    - is_active
    - is_verified
    - created_at
    - updated_at
        - metadata
emails:
    - value
    - user_id
    - verified
    - primary

phones:
    - value
    - user_id
    - verified
    - primary

devices:
    - ip
    - user_id - nullable
    - device_id
    - user_agent
    - last_login
    - is_active
    - created_at
    - updated_at

users_roles (many to many):
    - user_id
    - role_id

Access Token:
    {
    "sub": "user_id_or_unique_identifier", // User ID
    "iat": 1672531200, // Issued at (timestamp)
    "exp": 1672534800, // Expiration time (short-lived, e.g., 15 mins)
    "jti": "unique_access_token_id", // JWT ID for tracking in Redis
    "roles": ["admin", "user"], // User roles
    "scope": ["read", "write"], // Scopes for API access
        "metadata": {
            "ip": "192.168.1.1", // IP address (optional)
            "userAgent": "Mozilla/5.0" // User agent string (optional)
        }
    }

Refresh Token:
    {
    "session_id": "unique_session_id", // Optional: ties to a specific session
    "sub": "user_id", // User ID
    "iat": 1672531200, // Issued at (timestamp)
    "exp": 1675132800, // Expiration time (long-lived, e.g., 7 days)
    "jti": "unique_refresh_token_id", // JWT ID for tracking in Redis
        "metadata": {
            "ip": "192.168.1.1", // IP address (optional)
            "userAgent": "Mozilla/5.0" // User agent string (optional)
        }
    }

- last_login


Can generate a tokens for a users
Can generate api keys for a user

users -> roles -> permissions -> actions
users - organization -> roles -> permissions -> actions


- can create user
- send TOTP code
- verify TOTP code
- verify email
- Event For Completed Registration
- Event For Completed Email Verification

    // PasswordInitRequired     bool
    // PasswordChangeRequired   bool
    // UsernameChangeRequired   bool
    PasswordlessTokens


    ingot.registerUser({
        username: "mahmoud",
        password: "password",
        email: "m",
        phone: "123",
        metadata: {
            ip: "",
            userAgent: ""
        },
        roles: ["admin", "user"],
        permissions: ["read", "write"]
    })

    ingot.login({
        "method": "username",
        "payload": {
            "username": "mahmoud",
            "password": "password"
        }
    })


    ingot.login({
        "method": "phone",
        "payload": {
            "username": "email",
            "password": "password"
        }
    })

    ingot.login({
        "method": "totp",
        "payload": {
            "username": "mahmoud",
            "code": "123456"
        }
    })

    ingot.verifyEmail({
        "email": "m",
        "code": "123456"
    })

    ingot.subscribe("OTP.created", (event) => {
        console.log(event)
    })

    ingot.subscribe("OTP.verified", (event) => {
        console.log(event)
    })

    ingot.subscribe("User.created", (event) => {
        console.log(event)
    })
*/
