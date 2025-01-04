// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "email_status_enum"))]
    pub struct EmailStatusEnum;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "user_status_enum"))]
    pub struct UserStatusEnum;
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
    use super::sql_types::UserStatusEnum;

    users (user_uuid) {
        user_uuid -> Uuid,
        #[max_length = 50]
        username -> Varchar,
        password_hash -> Bytea,
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

diesel::allow_tables_to_appear_in_same_query!(
    emails,
    users,
);
