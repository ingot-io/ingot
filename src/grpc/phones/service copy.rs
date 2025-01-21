use argon2::{password_hash::Salt};
use serde_json::json;
use regex::Regex;
use uuid::Uuid;
use std::borrow::Borrow;
use std::sync::{Arc, Mutex};
use diesel::{PgConnection};
use tonic::{Request, Response, Status};

use crate::utils;
use crate::models;


pub mod users_v1 {
    tonic::include_proto!("ingot.api.users.v1"); // The string specified here must match the proto package name
}

pub use users_v1::users_server::{Users, UsersServer};
use users_v1::{ChangePasswordRequest, UpdateUserRequest, GetUserByUsernameRequest, CheckPasswordRequest, CheckPasswordResponse, CreateUserRequest, DeleteUserRequest, GetUserByIdRequest, UserResponse};

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
}