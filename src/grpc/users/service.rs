use argon2::password_hash::Salt;
use serde_json::json;
use regex::Regex;
use uuid::Uuid;
use std::borrow::Borrow;
use std::sync::{Arc, Mutex};
use diesel::PgConnection;
use tonic::{Request, Response, Status};

use crate::utils;
use crate::models;

use super::v1::{ChangePasswordRequest, UpdateUserRequest, GetUserByUsernameRequest, CheckPasswordRequest, CheckPasswordResponse, CreateUserRequest, DeleteUserRequest, GetUserByIdRequest, UserResponse};
use super::v1::users_server::Users;

pub struct UsersService {
    database: Arc<Mutex<PgConnection>>,
}

impl UsersService {
    pub fn new(database: Arc<Mutex<PgConnection>>) -> Self {
        Self {
            database,
        }
    }
}

#[tonic::async_trait]
impl Users for UsersService {
    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<UserResponse>, Status> {
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
        let hashed_password = utils::hash_password(&user.password, salt).unwrap();

        let user = models::NewUser {
            username: user.username.to_lowercase(),
            password_hash: hashed_password,
            metadata: json!({}),
        };

        let database = self.database.lock();
        
        let user = models::User::create(&mut database.unwrap(), user)
            .map_err(|e| Status::internal(format!("Error creating user: {}", e)))?;

        Ok(Response::new(UserResponse {
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

    async fn check_user_password(
        &self,
        request: Request<CheckPasswordRequest>
    ) -> Result<Response<CheckPasswordResponse>, Status> {
        let request = request.into_inner();
        let mut database = self.database.lock().unwrap();
        
        let user = models::User::find_user_uuid(&mut database, Uuid::parse_str(&request.id).unwrap())
            .map_err(|e| Status::internal(format!("Error finding user: {}", e)))?;
  
        let valid = utils::verify_password(&request.password, &user.password_hash);

        let valid = match valid {
            Ok(valid) => valid,
            Err(e) => return Err(Status::internal(format!("Error verifying password: {}", e))),
        };

        Ok(Response::new(CheckPasswordResponse {
            valid
        }))
    }

    async fn get_user_by_id( &self,
        request: Request<GetUserByIdRequest>
    ) -> Result<Response<UserResponse>, Status> {
        let request = request.into_inner();
        let mut database = self.database.lock().unwrap();
        
        let user = models::User::find_user_uuid(&mut database, Uuid::parse_str(&request.id).unwrap())
            .map_err(|e| Status::internal(format!("Error finding user: {}", e)))?;

        Ok(Response::new(UserResponse {
            id: user.user_uuid.to_string(),
            username: user.username,
            status: user.status as i32,
            is_verified: user.is_verified,
            onboarded: user.onboarded,
        }))
    }

    async fn delete_user(
        &self,
        request: Request<DeleteUserRequest>,
    ) -> Result<Response<()>, Status> {
        let request = request.into_inner();
        let mut database = self.database.lock().unwrap();

        // Check if user Exist 
        
        models::User::delete_user(&mut database, Uuid::parse_str(&request.id).unwrap())
            .map_err(|e| Status::internal(format!("Error deleting user: {}", e)))?;

        Ok(Response::new(()))
    }


    async fn change_password(
        &self,
        request: Request<ChangePasswordRequest>,
    ) -> Result<Response<()>, Status> {
        let request = request.into_inner();
        let mut database = self.database.lock().unwrap();

        // Validate new password requirements
        let password = request.new_password.borrow();

        if request.new_password.len() < 8 {
            return Err(Status::invalid_argument("Password must be at least 8 characters"));
        }

        if request.new_password.len() > 64 {
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

        let user = models::User::find_user_uuid(&mut database, Uuid::parse_str(&request.id).unwrap())
            .map_err(|e| Status::internal(format!("Error finding user: {}", e)))?;

        // Verify old password
        let is_valid = utils::verify_password(&request.current_password, &user.password_hash)
            .map_err(|e| Status::internal(format!("Error verifying password: {}", e)))?;
        if !is_valid {
            return Err(Status::permission_denied("Invalid old password"));
        }

        // Hash and update new password
        let salt_str = "YmFkIHNhbHQh";
        let salt: Salt = salt_str.try_into().unwrap();
        let new_hash = utils::hash_password(&request.new_password, salt).unwrap();

        models::User::update_password(&mut database, user.user_uuid, new_hash)
            .map_err(|e| Status::internal(format!("Error updating password: {}", e)))?;

        Ok(Response::new(()))
    }


    async fn get_user_by_username(
        &self,
        request: Request<GetUserByUsernameRequest>,
    ) -> Result<Response<UserResponse>, Status> {
        let request = request.into_inner();
        let mut database = self.database.lock().unwrap();
        
        let user = models::User::find_by_username(&mut database, request.username)
            .map_err(|e| Status::internal(format!("Error finding user: {}", e)))?;

        Ok(Response::new(UserResponse {
            id: user.user_uuid.to_string(),
            username: user.username,
            status: user.status as i32,
            is_verified: user.is_verified,
            onboarded: user.onboarded,
        }))
    }


    // UpdateUserRequest is used to update user details.
    async fn update_user( &self,
        request: Request<UpdateUserRequest>,
    ) -> Result<Response<UserResponse>, Status> {
        let request = request.into_inner();
        let mut database = self.database.lock().unwrap();

        // Check if user exists
        let user_uuid = Uuid::parse_str(&request.id)
            .map_err(|_| Status::invalid_argument("Invalid UUID format"))?;

        let user = models::User::find_user_uuid(&mut database, user_uuid)
            .map_err(|e| Status::internal(format!("Error finding user: {}", e)))?;

        // Save the updated user
        let updated_user = models::User::update(&mut database, user)
            .map_err(|e| Status::internal(format!("Error updating user: {}", e)))?;

        Ok(Response::new(UserResponse {
            id: updated_user.user_uuid.to_string(),
            username: updated_user.username,
            status: updated_user.status as i32,
            is_verified: updated_user.is_verified,
            onboarded: updated_user.onboarded,
        }))
    }
}