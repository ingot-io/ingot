use diesel::{
    Insertable, PgConnection, Queryable, RunQueryDsl, Selectable, SelectableHelper
};
use uuid::Uuid;
use crate::schema::users;
use serde_json::Value;

// use crate::sql_types::JsonbValue;

// sql_types, ExpressionMethods, Insertable, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper
// use crate::schema::sql_types;
// use diesel::sql_types::SqlType;

// use diesel::serialize::{self, ToSql, IsNull, Output};
// use diesel::pg::Pg;
// use std::io::Write;

// #[derive(Debug, Clone, Copy, PartialEq, Eq, diesel_derive_enum::DbEnum)]
// #[DieselType = "user_status_enum"] // Must match the SQL type name in your database
// pub enum UserStatusEnum {
    // Active,
    // Inactive,
    // Suspended,
// }

#[derive(Debug, PartialEq, Clone, Eq, diesel_derive_enum::DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::UserStatusEnum"]
pub enum UserStatusEnum {
    Active,
    Inactive,
    Banned,
    Deleted
}


#[derive(Queryable, Selectable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    #[diesel(sql_type = sql_types::Uuid)]
    pub user_uuid: Uuid,
    pub username: String,
    pub password_hash: Vec<u8>,
    pub is_verified: bool,
    pub onboarded: bool,
    pub status: UserStatusEnum,
    pub metadata: Value, // JSONB field
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub password_hash: Vec<u8>,
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
}