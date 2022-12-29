use actix_web::web;
use diesel::{PgConnection, r2d2::ConnectionManager};
use r2d2::PooledConnection;
use serde::Serialize;

use crate::AppState;

pub mod main;
pub mod accounts;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponse<T, U> {
    pub is_array: bool,
    pub data: T,
    pub metadata: U,
}

fn default_none<T>() -> Option<T> {
    None
}

fn default_false() -> bool {
    false
}

fn default_paging_page() -> i64 {
    1
}

fn default_paging_per_page() -> i64 {
    20
}

fn get_conn<'a>(app_state: &'a web::Data<AppState>) -> PooledConnection<ConnectionManager<PgConnection>> {
    let pool = app_state.db_pool.clone();
    pool.get().unwrap()
}

fn paging_calculate_offset(page: i64, per_page: i64) -> i64 {
    if page > 0 {
        return (page - 1) * per_page
    }

    0
}