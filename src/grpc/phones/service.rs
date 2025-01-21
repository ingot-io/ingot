use std::sync::{Arc, Mutex};
use diesel::{PgConnection};
use serde_json::json;
use tonic::{Request, Response, Status};

use crate::models;

use uuid::Uuid;
use super::v1::phones_server::Phones;
use super::v1::{CreatePhoneRequest, Phone, PhoneStatus};

pub struct PhonesService {
    database: Arc<Mutex<PgConnection>>,
}

impl PhonesService {
    pub fn new(database: Arc<Mutex<PgConnection>>) -> Self {
        Self {
            database,
        }
    }
}

#[tonic::async_trait]
impl Phones for PhonesService {
    async fn create_phone(&self,
        request: Request<CreatePhoneRequest>) -> Result<Response<Phone>, Status> {
        let inputs = request.into_inner();
        let mut database = self.database.lock().unwrap();    
        
        let user_uuid = Uuid::parse_str(&inputs.user_id).unwrap();

        let new_phone = models::NewPhone {
            user_uuid: user_uuid,
            country_code: inputs.country_code,
            number: inputs.number,
            type_: inputs.r#type,
            status: models::PhoneStatusEnum::Unverified,
            metadata: json!({}),
        };

        let phone = models::Phone::create(&mut database, new_phone)
            .map_err(|e| Status::internal(e.to_string()))?;
        

        let full_number = phone.full_number.map(|n| n.to_string());

        Ok(Response::new(Phone{
            user_id: phone.user_uuid.to_string(),
            phone_id: phone.phone_uuid.to_string(),
            country_code: phone.country_code,
            number: phone.number.to_string(),
            full_number: full_number.unwrap(),
            r#type: phone.type_.unwrap_or_default().to_string(),
            is_primary: false,
            is_verified: false,
            status: PhoneStatus::Unverified as i32,
        }))

    }
}