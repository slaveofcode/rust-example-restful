use actix_web::{web, post, Responder, Result, http::StatusCode};
use diesel::{insert_into, RunQueryDsl};
use serde::{Serialize, Deserialize};
use crate::{schema::accounts::dsl::{accounts}, db::account::{Account, AccountNew}, controllers::{get_conn, default_false, ApiResponse}, AppState};

#[derive(Serialize)]
struct Metadata {
    message: Option<String>
}

#[derive(Deserialize)]
struct AccountCreateParam {
    name: String,
    #[serde(rename = "isActive", default = "default_false")]
    is_active: bool,
}

#[post("")]
async fn create_account(post_data: web::Json<AccountCreateParam>, app_state: web::Data<AppState>) -> Result<impl Responder> {
    let conn = &mut get_conn(&app_state);

    let acc = AccountNew {
        name: &post_data.name,
        is_active: &post_data.is_active,
    };

    let new_acc = insert_into(accounts)
        .values(acc)
        .get_result::<Account>(conn);

    if let Err(msg) = new_acc {
        return Ok((web::Json(ApiResponse::<Option<Account>, Metadata> {
            is_array: false,
            data: None,
            metadata: Metadata {
                message: Some(msg.to_string()),
            }
        }), StatusCode::INTERNAL_SERVER_ERROR));
    }

    Ok((web::Json(ApiResponse::<Option<Account>, Metadata> {
        is_array: false,
        data: new_acc.ok(),
        metadata: Metadata { message: None }
    }), StatusCode::CREATED))
}