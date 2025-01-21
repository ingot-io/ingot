use diesel::{
    ExpressionMethods, Insertable, PgConnection, QueryDsl, Queryable, RunQueryDsl, Selectable, SelectableHelper,
    sql_types::Timestamptz
};
use uuid::Uuid;
use crate::schema::{users, devices, sessions};
use serde_json::Value;
use ipnet::IpNet;

#[derive(Debug, PartialEq, Clone, Eq, diesel_derive_enum::DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::UserStatusEnum"]
pub enum UserStatusEnum {
    Active,
    Inactive,
    Banned,
    Deleted
}

#[derive(Debug, PartialEq, Clone, Eq, diesel_derive_enum::DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::DeviceTypeEnum"]
pub enum DeviceTypeEnum {
    Desktop,
    Laptop,
    Mobile,
    Tablet,
    Iot,
    Wearable,
    GamingConsole,
    SmartTv,
    Car,
    VrHeadset,
    EReader,
    Server,
    Other
}

impl DeviceTypeEnum {
    pub fn from_device_family(family: &str) -> Self {
        match family.to_lowercase().as_str() {
            "desktop" => DeviceTypeEnum::Desktop,
            "laptop" => DeviceTypeEnum::Laptop,
            "mobile" | "smartphone" | "iphone" | "android" => DeviceTypeEnum::Mobile,
            "tablet" | "ipad" => DeviceTypeEnum::Tablet,
            "iot" | "embedded" => DeviceTypeEnum::Iot,
            "wearable" | "smartwatch" => DeviceTypeEnum::Wearable,
            "gamingconsole" | "playstation" | "xbox" => DeviceTypeEnum::GamingConsole,
            "smarttv" | "tv" => DeviceTypeEnum::SmartTv,
            "car" | "vehicle" => DeviceTypeEnum::Car,
            "vrheadset" | "virtualreality" => DeviceTypeEnum::VrHeadset,
            "ereader" | "kindle" => DeviceTypeEnum::EReader,
            "server" => DeviceTypeEnum::Server,
            "spider" | "bot" | "crawler" => DeviceTypeEnum::Other, // Handle web crawlers
            _ => DeviceTypeEnum::Other, // Default case
        }
    }
}

#[derive(Debug, PartialEq, Clone, Eq, diesel_derive_enum::DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::OsEnum"]
pub enum OsEnum {
    Windows,
    Macos,
    Linux,
    Android,
    Ios,
    ChromeOs,
    Tizen,
    FireOs,
    Watchos,
    Webos,
    Freebsd,
    Other
}

impl OsEnum {
    pub fn from_device_family(family: &str) -> Self {
        match family.to_lowercase().as_str() {
            "Windows" => OsEnum::Windows,
            "Mac OS X" => OsEnum::Macos,
            "Linux" => OsEnum::Linux,
            "Android" => OsEnum::Android,
            "iOS" => OsEnum::Ios,
            "Chrome OS" => OsEnum::ChromeOs,
            "Tizen" => OsEnum::Tizen,
            "Fire OS" => OsEnum::FireOs,
            "watchOS" => OsEnum::Watchos,
            "webOS" => OsEnum::Webos,
            "FreeBSD" => OsEnum::Freebsd,
            _ => OsEnum::Other,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Eq, diesel_derive_enum::DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::DeviceStatusEnum"]
pub enum DeviceStatusEnum {
    Active,
    Inactive,
    Suspended,
    Banned,
    Pending,
    Retired,
    Lost,
    Stolen,
    Maintenance,
    Offline
}


#[derive(Queryable, Selectable, Clone)]
#[diesel(table_name = devices)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Device {
    pub device_uuid: Uuid,
    pub device_type: DeviceTypeEnum,
    pub os: OsEnum,
    pub os_version: Option<String>,
    pub browser: Option<String>,
    pub browser_version: Option<String>,
    pub ip_address: IpNet,
    pub location: Option<Value>,
    pub status: DeviceStatusEnum,
    pub last_used_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    pub mac_address: Option<String>,
    pub notification_token: Option<String>,
    pub notification_enabled: Option<bool>,
    pub metadata: Value
}

#[derive(Insertable)]
#[diesel(table_name = devices)]
pub struct NewDevice {
    pub device_type: DeviceTypeEnum,
    pub os: OsEnum,
    pub os_version: Option<String>,
    pub browser: Option<String>,
    pub browser_version: Option<String>,
    // #[diesel(sql_type = diesel::sql_types::Inet)]
    pub ip_address: IpNet,
    pub location: Option<Value>,
    pub status: DeviceStatusEnum
}

impl Device {
    pub fn create(
        conn: &mut PgConnection,
        new_device: NewDevice,
    ) -> Result<Device, diesel::result::Error> {
        diesel::insert_into(devices::table)
            .values(new_device)
            .returning(Device::as_returning())
            .get_result(conn)   
    }

    pub fn find_by_uuid(
        conn: &mut PgConnection,
        device_uuid: Uuid,
    ) -> Result<Device, diesel::result::Error> {
        devices::table
            .filter(devices::device_uuid.eq(device_uuid))
            .select(Device::as_select())
            .first(conn)   
    }
}

#[derive(Queryable, Selectable, Clone)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    // #[prost(string, tag = "1")]
    // #[diesel(sql_type = sql_types::Uuid)]
    pub user_uuid: Uuid,
    
    // #[prost(string, tag = "2")]
    pub username: String,
    
    pub password_hash: String,
    
    // #[prost(string, tag = "3")]
    pub is_verified: bool,

    // #[prost(bool, tag = "4")]
    pub onboarded: bool,

    // #[prost(enumeration = "UserStatusEnum", tag = "6")]
    pub status: UserStatusEnum,

    // #[prost(message, tag = "7")]
    // #[diesel(sql_type = Jsonb)]
    pub metadata: Value, // JSONB field
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub password_hash: String,
    pub metadata: Value,
}


impl User {
    pub fn create(
        conn: &mut PgConnection,
        new_user: NewUser,
    ) -> Result<User, diesel::result::Error> {
        diesel::insert_into(users::table)
            .values(new_user)
            .returning(User::as_returning())
            .get_result(conn)   
    }


    pub fn find_user_uuid(
        conn: &mut PgConnection,
        user_uuid: Uuid,
    ) -> Result<User, diesel::result::Error> {
        users::table
            .filter(users::user_uuid.eq(user_uuid))
            .select(User::as_select())
            .first(conn)
    }


    pub fn delete_user(
        conn: &mut PgConnection,
        user_uuid: Uuid,
    ) -> Result<bool, diesel::result::Error> {
        let deleted = diesel::delete(users::table)
            .filter(users::user_uuid.eq(user_uuid))
            .execute(conn)?;
        Ok(deleted > 0)
    }

    pub fn update_password(
        conn: &mut PgConnection,
        user_uuid: Uuid,
        hashed_password: String
    ) -> Result<bool, diesel::result::Error> {
        let updated = diesel::update(users::table)
            .filter(users::user_uuid.eq(user_uuid))
            .set(users::password_hash.eq(hashed_password))
            .execute(conn)?;
        Ok(updated > 0)
    }

    pub fn find_by_username(
        conn:  &mut PgConnection,
        username: String
    ) -> Result<User, diesel::result::Error> {
        users::table
            .filter(users::username.eq(username))
            .select(User::as_select())
            .first(conn)
    }

    pub fn update(
        conn:  &mut PgConnection,
        user: User,
    ) -> Result<User, diesel::result::Error> {
        diesel::update(users::table)
            .filter(users::user_uuid.eq(user.user_uuid))
            .set((
                users::username.eq(user.username),
            ))
            .returning(User::as_returning())
            .get_result(conn)
    }
}


#[derive(Queryable, Selectable, Clone)]
#[diesel(table_name = sessions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Session {
    pub session_uuid: Uuid,
    pub user_uuid: Uuid,
    pub device_uuid: Option<Uuid>,
    pub ip_address: IpNet,
    pub metadata: Value,
    pub last_accessed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub is_active: bool,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub invalidated_at: Option<chrono::DateTime<chrono::Utc>>,
    pub revoked_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>
}

#[derive(Insertable)]
#[diesel(table_name = sessions)]
pub struct NewSession {
    pub user_uuid: Uuid,
    pub device_uuid: Uuid,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub ip_address: IpNet,
    pub metadata: Value,
}

impl Session {
    pub fn create(
        conn: &mut PgConnection,
        new_session: NewSession,
    ) -> Result<Session, diesel::result::Error> {
        diesel::insert_into(sessions::table)
            .values(new_session)
            .returning(Session::as_returning())
            .get_result(conn)
    }

    pub fn find_by_uuid(
        conn: &mut PgConnection,
        session_uuid: Uuid,
    ) -> Result<Session, diesel::result::Error> {
        sessions::table
            .filter(sessions::session_uuid.eq(session_uuid))
            .select(Session::as_select())
            .first(conn)
    }

    pub fn delete(
        conn: &mut PgConnection,
        session_uuid: Uuid,
    ) -> Result<bool, diesel::result::Error> {
        let deleted = diesel::delete(sessions::table)
            .filter(sessions::session_uuid.eq(session_uuid))
            .execute(conn)?;
        Ok(deleted > 0)
    }
}