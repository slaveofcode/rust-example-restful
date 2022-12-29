use actix_web::{web, get, Responder, Result};
use diesel::{
    QueryDsl,
    RunQueryDsl,
    ExpressionMethods,
    associations::HasTable,
};
use serde::{Serialize, Deserialize};
use crate::{
    AppState,
    schema::accounts::dsl::{accounts, is_active, created_at, deleted_at},
    db::account::Account,
    controllers::{
        default_none, 
        get_conn, 
        default_paging_page, 
        default_paging_per_page, 
        paging_calculate_offset, 
        ApiResponse
    }
};

#[derive(Serialize)]
struct AccountListResp {
    accounts: Vec<Account>
}

#[derive(Serialize)]
struct ListMetadataResp {
    page: i64,
    per_page: i64,
    total: i64,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct QueryGetAll {
    #[serde(default = "default_none")]
    is_active: Option<bool>,

    #[serde(default = "default_paging_page")]
    page: i64,

    #[serde(default = "default_paging_per_page")]
    per_page: i64,
}

#[get("")]
async fn list_accounts(query: web::Query<QueryGetAll>, app_state: web::Data<AppState>) -> Result<impl Responder> {
    let conn = &mut get_conn(&app_state);

    let mut query_accounts = accounts::table().into_boxed().filter(deleted_at.is_null());
    let mut query_accounts_count = accounts::table().into_boxed().filter(deleted_at.is_null());

    if let Some(active) = query.is_active {
        query_accounts = query_accounts.filter(is_active.eq(active));
        query_accounts_count = query_accounts_count.filter(is_active.eq(active));
    }
    
    query_accounts = query_accounts.offset(paging_calculate_offset(query.page, query.per_page));
    query_accounts = query_accounts.limit(query.per_page);

    let list: Vec<Account> = query_accounts.order(created_at.desc())
        .load::<Account>(conn)
        .expect("unable to load list account");
    let total = query_accounts_count
        .count()
        .get_result::<i64>(conn)
        .unwrap();
    
    Ok(web::Json(ApiResponse::<Vec<Account>, ListMetadataResp> {
        is_array: true,
        data: list,
        metadata: ListMetadataResp {
            page: query.page,
            per_page: query.per_page,
            total,
        }
    }))
}