mod models;
mod schema;
mod grpc;
mod utils;

use std::env;
use dotenvy::dotenv;
use diesel::{PgConnection, Connection};
use tonic::transport::Server;

// use crate::grpc::users::v1::;

pub fn connect_db() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
    .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let database = std::sync::Arc::new(std::sync::Mutex::new(connect_db()));
    
    println!("Starting server...");
    
    let addr = "[::1]:50051".parse()?;
    
    let users_service = grpc::users::service::UsersService::new(database.clone());
    let auth_service = grpc::auth::service::AuthService::new(database.clone());
    let device_service = grpc::device::service::DevicesService::new(database.clone());
    let emails_service = grpc::emails::service::EmailsService::new(database.clone());
    let phone_service = grpc::phones::service::PhonesService::new(database.clone());

    Server::builder()
    .add_service(grpc::users::v1::users_server::UsersServer::new(users_service))
    .add_service(grpc::auth::v1::auth_server::AuthServer::new(auth_service))
    .add_service(grpc::device::v1::devices_server::DevicesServer::new(device_service))
    .add_service(grpc::emails::v1::emails_server::EmailsServer::new(emails_service))
    .add_service(grpc::phones::v1::phones_server::PhonesServer::new(phone_service))
    .serve(addr)
    .await?;

    Ok(())
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
