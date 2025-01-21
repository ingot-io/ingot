pub mod users;
pub mod device;
pub mod auth;
pub mod emails;
pub mod phones;

// pub mod auth {
//     pub mod v1 {
//         tonic::include_proto!("ingot.api.auth.v1");
//     }
// }


// pub mod devices {
//     pub mod v1 {
//         tonic::include_proto!("ingot.api.devices.v1");
//     }
// }

// // Re-export custom service implementations
// pub mod services {
//     pub mod auth {
//         pub use super::super::users::v1::{};
//     }
// }