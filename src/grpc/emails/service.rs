use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use diesel::{PgConnection};
use serde_json::json;
use tonic::{Request, Response, Status};

use jsonwebtoken::{encode, Header, Algorithm, EncodingKey};

use chrono::{DateTime, Utc};
use crate::models;

use prost_types::Timestamp;

use uuid::Uuid;
use super::v1::emails_server::Emails;
use super::v1::{CreateEmailRequest, Email, EmailResponse, EmailStatus};

pub struct EmailsService {
    database: Arc<Mutex<PgConnection>>,
}

impl EmailsService {
    pub fn new(database: Arc<Mutex<PgConnection>>) -> Self {
        Self {
            database,
        }
    }
}


fn convert_to_prost_timestamp(dt: DateTime<Utc>) -> Timestamp {
    Timestamp {
        seconds: dt.timestamp(),  // Extract seconds as i64
        nanos: dt.timestamp_subsec_nanos() as i32,  // Extract nanoseconds as i32
    }
}

#[tonic::async_trait]
impl Emails for EmailsService {
    async fn create_email(&self,
        request: Request<CreateEmailRequest>) -> Result<Response<EmailResponse>, Status> {
        let inputs = request.into_inner();
        let mut database = self.database.lock().unwrap();    
        
        let user_uuid = Uuid::parse_str(&inputs.user_id).unwrap();

        let new_email = models::NewEmail {
            user_uuid: user_uuid,
            metadata: json!({}),
            value: inputs.email,
            status: models::EmailStatusEnum::Unverified,
        };

        let email = models::Email::create(&mut database, new_email)
            .map_err(|e| Status::internal(e.to_string()))?;
        
        Ok(Response::new(EmailResponse{
            email:  Some (Email{
                id: email.user_uuid.to_string(),
                email: email.value,
                is_verified: false,
                status: EmailStatus::Unverified as i32,
                is_primary: false,
            }),
            success: true,
            message: "".to_string()
        }))

    }
}