use actix_web::{put, web, Result, Responder, http::StatusCode};
use chrono::{Utc};
use serde::{Serialize, Deserialize};
use diesel::{prelude::*};
use crate::{
    AppState,
    db::account::{Account, AccountUpdate},
    controllers::{get_conn, ApiResponse},
    schema::accounts::dsl::{accounts, id},
};

#[derive(Serialize)]
struct Metadata {
    message: Option<String>
}

#[derive(Deserialize)]
struct AccountUpdateParam {
    name: Option<String>,
    #[serde(rename = "isActive")]
    is_active: Option<bool>
}

#[put("/{account_id}")]
async fn update_account(put_data: web::Json<AccountUpdateParam>, path_param: web::Path<(i32,)>, app_state: web::Data<AppState>) -> Result<impl Responder> {
    let conn = &mut get_conn(&app_state);
    let account = accounts.filter(id.eq(path_param.0)).first::<Account>(conn);

    if let Err(msg) = account {
        return Ok((web::Json(ApiResponse::<Option<Account>, Metadata> {
            is_array: false,
            data: None,
            metadata: Metadata {
                message: Some(msg.to_string())
            }
        }), StatusCode::BAD_REQUEST))
    }

    let updated_account = diesel::update(accounts.filter(id.eq(path_param.0)))
        .set::<AccountUpdate>(AccountUpdate {
            name: put_data.name.as_deref(),
            is_active: put_data.is_active.as_ref(),
            updated_at: Utc::now().naive_utc(),
        })
        .get_result::<Account>(conn);

    if let Err(msg) = updated_account {
        return Ok((web::Json(ApiResponse::<Option<Account>, Metadata> {
            is_array: false,
            data: None,
            metadata: Metadata {
                message: Some(msg.to_string())
            }
        }), StatusCode::BAD_REQUEST))
    }

    Ok((web::Json(ApiResponse::<Option<Account>, Metadata> {
        is_array: false,
        data: updated_account.ok(),
        metadata: Metadata {
            message: None
        }
    }), StatusCode::OK))
}