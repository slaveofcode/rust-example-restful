use chrono::NaiveDateTime;
use diesel::{prelude::{Queryable, Insertable}};
use serde::Serialize;
use crate::schema::roles;

#[derive(Debug, Queryable, PartialEq, Serialize)]
#[diesel(table_name = roles)]
pub struct Role {
    pub id: i32,
    pub role: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    #[serde(skip)]
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = roles)]
pub struct RoleNew<'a> {
    pub role: &'a str,
}
