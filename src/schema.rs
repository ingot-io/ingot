// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "device_status_enum"))]
    pub struct DeviceStatusEnum;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "device_type_enum"))]
    pub struct DeviceTypeEnum;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "email_status_enum"))]
    pub struct EmailStatusEnum;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "membership_invitation_status_enum"))]
    pub struct MembershipInvitationStatusEnum;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "membership_status_enum"))]
    pub struct MembershipStatusEnum;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "os_enum"))]
    pub struct OsEnum;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "phone_status_enum"))]
    pub struct PhoneStatusEnum;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "user_status_enum"))]
    pub struct UserStatusEnum;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::DeviceTypeEnum;
    use super::sql_types::OsEnum;
    use super::sql_types::DeviceStatusEnum;

    devices (device_uuid) {
        device_uuid -> Uuid,
        device_type -> DeviceTypeEnum,
        os -> OsEnum,
        #[max_length = 50]
        os_version -> Nullable<Varchar>,
        #[max_length = 100]
        browser -> Nullable<Varchar>,
        #[max_length = 50]
        browser_version -> Nullable<Varchar>,
        ip_address -> Inet,
        location -> Nullable<Jsonb>,
        status -> DeviceStatusEnum,
        last_used_at -> Nullable<Timestamptz>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        #[max_length = 17]
        mac_address -> Nullable<Varchar>,
        notification_token -> Nullable<Text>,
        notification_enabled -> Nullable<Bool>,
        metadata -> Jsonb,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::EmailStatusEnum;

    emails (email_uuid) {
        email_uuid -> Uuid,
        user_uuid -> Uuid,
        #[max_length = 255]
        value -> Varchar,
        status -> EmailStatusEnum,
        verified_at -> Nullable<Timestamp>,
        bounced_at -> Nullable<Timestamp>,
        is_primary -> Bool,
        is_verified -> Bool,
        metadata -> Jsonb,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::MembershipStatusEnum;
    use super::sql_types::MembershipInvitationStatusEnum;

    membership (membership_uuid) {
        membership_uuid -> Uuid,
        user_uuid -> Uuid,
        org_uuid -> Uuid,
        role_uuid -> Uuid,
        metadata -> Nullable<Jsonb>,
        status -> MembershipStatusEnum,
        invitation_status -> Nullable<MembershipInvitationStatusEnum>,
        joined_at -> Nullable<Timestamptz>,
        last_active_at -> Nullable<Timestamptz>,
        expired_at -> Nullable<Timestamptz>,
        leave_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    organizations (org_uuid) {
        org_uuid -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    permissions (permission_uuid) {
        permission_uuid -> Uuid,
        #[max_length = 255]
        permission_key -> Varchar,
        description -> Nullable<Text>,
        created_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::PhoneStatusEnum;

    phones (phone_uuid) {
        phone_uuid -> Uuid,
        user_uuid -> Uuid,
        #[max_length = 5]
        country_code -> Varchar,
        #[max_length = 20]
        number -> Varchar,
        #[max_length = 30]
        full_number -> Nullable<Varchar>,
        #[sql_name = "type"]
        #[max_length = 20]
        type_ -> Nullable<Varchar>,
        status -> PhoneStatusEnum,
        verified_at -> Nullable<Timestamp>,
        bounced_at -> Nullable<Timestamp>,
        is_primary -> Bool,
        is_verified -> Bool,
        metadata -> Jsonb,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    role_permissions (role_uuid, permission_uuid) {
        role_uuid -> Uuid,
        permission_uuid -> Uuid,
        granted_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    roles (role_uuid) {
        role_uuid -> Uuid,
        #[max_length = 100]
        role_name -> Varchar,
        description -> Nullable<Text>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    sessions (session_uuid) {
        session_uuid -> Uuid,
        user_uuid -> Uuid,
        device_uuid -> Nullable<Uuid>,
        ip_address -> Inet,
        metadata -> Jsonb,
        last_accessed_at -> Nullable<Timestamptz>,
        is_active -> Bool,
        expires_at -> Timestamptz,
        invalidated_at -> Nullable<Timestamptz>,
        revoked_at -> Nullable<Timestamptz>,
        created_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    user_roles (user_uuid, role_uuid) {
        user_uuid -> Uuid,
        role_uuid -> Uuid,
        assigned_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::UserStatusEnum;

    users (user_uuid) {
        user_uuid -> Uuid,
        #[max_length = 50]
        username -> Varchar,
        #[max_length = 255]
        password_hash -> Varchar,
        status -> UserStatusEnum,
        is_verified -> Bool,
        onboarded -> Bool,
        metadata -> Jsonb,
        archived_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(emails -> users (user_uuid));
diesel::joinable!(membership -> organizations (org_uuid));
diesel::joinable!(membership -> roles (role_uuid));
diesel::joinable!(membership -> users (user_uuid));
diesel::joinable!(phones -> users (user_uuid));
diesel::joinable!(role_permissions -> permissions (permission_uuid));
diesel::joinable!(role_permissions -> roles (role_uuid));
diesel::joinable!(sessions -> users (user_uuid));
diesel::joinable!(user_roles -> roles (role_uuid));
diesel::joinable!(user_roles -> users (user_uuid));

diesel::allow_tables_to_appear_in_same_query!(
    devices,
    emails,
    membership,
    organizations,
    permissions,
    phones,
    role_permissions,
    roles,
    sessions,
    user_roles,
    users,
);
