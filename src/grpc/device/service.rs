use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use diesel::{PgConnection};
use tonic::{Request, Response, Status};

use jsonwebtoken::{encode, Header, Algorithm, EncodingKey};

use crate::utils;
use crate::models;

use uaparser::{Parser, UserAgentParser};
use ipnet::IpNet; // Import ipnet types
use std::net::IpAddr;


use super::v1::devices_server::Devices;
use super::v1::{DeviceResponse, Device, DeviceType, Os, DeviceStatus, CreateDeviceRequest};

pub struct DevicesService {
    database: Arc<Mutex<PgConnection>>,
}

impl DevicesService {
    pub fn new(database: Arc<Mutex<PgConnection>>) -> Self {
        Self {
            database,
        }
    }
}

#[tonic::async_trait]
impl Devices for DevicesService {
    async fn create_device( &self,
        request: Request<CreateDeviceRequest>,
    ) -> Result<Response<DeviceResponse>, Status> {
       let inputs = request.into_inner();
       
        // Initialize the parser
        let parser = UserAgentParser::builder().build_from_yaml("config/regexes.yaml").unwrap();

        let client = parser.parse(&inputs.user_agent);

        let device_type = models::DeviceTypeEnum::from_device_family(&client.device.family);
        let os = models::OsEnum::from_device_family(&client.os.family);

        let database = self.database.lock();

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

        let new_device = models::NewDevice {
            browser: Some(client.user_agent.family.to_string()),
            browser_version: Some(client.user_agent.family.to_string()),
            os_version: Some(client.os.family.to_string()),
            device_type: device_type,
            os: os,
            status: models::DeviceStatusEnum::Active,
            ip_address: ip_address.unwrap(),
            location: Some(serde_json::json!({
                "latitude": 37.7749,
                "longitude": -122.4194,
                "city": "San Francisco",
                "country": "US"
            }))
        };

        let device = models::Device::create(&mut database.unwrap(), new_device)
        .map_err(|e| Status::internal(format!("Error creating device: {}", e)))?;

        Ok(Response::new(DeviceResponse {
            success: true,
            device: Some(
                Device {
                    id: device.device_uuid.to_string(),
                    browser: device.browser.unwrap_or_default(),
                    browser_version: device.browser_version.unwrap_or_default(),
                    os:  Os::Unspecified.into(),
                    os_version: device.os_version.unwrap_or_default(),
                    device_type: DeviceType::Unspecified.into(),
                    status: DeviceStatus::Active.into(),
                    location: HashMap::new(),
                    ip_address: device.ip_address.to_string(),
                    mac_address: "".to_string(),
                    metadata: HashMap::new(),
                    notification_enabled: false,
                    notification_token: "Fake".to_string()
                }
            )
        }))
    }
}