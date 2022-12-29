use chrono::NaiveDateTime;
use diesel::{prelude::{Queryable, Insertable}, AsChangeset};
use serde::Serialize;
use crate::schema::accounts;

#[derive(Debug, Queryable, PartialEq, Serialize)]
#[diesel(table_name = accounts)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub id: i32,
    pub name: String,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,

    #[serde(skip)]
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = accounts)]
pub struct AccountNew<'a> {
    pub name: &'a str,
    pub is_active: &'a bool,
}

#[derive(AsChangeset)]
#[diesel(table_name = accounts)]
pub struct AccountUpdate<'a> {
    pub name: Option<&'a str>,
    pub is_active: Option<&'a bool>,
    pub updated_at: NaiveDateTime,
}

#[derive(AsChangeset)]
#[diesel(table_name = accounts)]
pub struct AccountDelete {
    pub updated_at: NaiveDateTime,
    pub deleted_at: NaiveDateTime,
}