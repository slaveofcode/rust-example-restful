use actix_web::{get, web, Result, Responder, http::StatusCode};
use serde::Serialize;
use diesel::{prelude::*, result::Error};
use crate::{
    db::account::Account, 
    AppState, 
    controllers::{get_conn, ApiResponse}, 
    schema::accounts::dsl::{accounts, id, deleted_at},
};

#[derive(Serialize)]
struct Metadata {
    message: Option<String>
}

#[get("/{account_id}")]
async fn detail_account(path_param: web::Path<(i32,)>, app_state: web::Data<AppState>) -> Result<impl Responder> {
    let conn = &mut get_conn(&app_state);

    let account = accounts
        .filter(deleted_at.is_null())
        .filter(id.eq(path_param.0))
        .first(conn);

    if let Err(err) = account {
        if err == Error::NotFound {
            return Ok((web::Json(ApiResponse::<Option<Account>, Metadata> {
                is_array: false,
                data: None,
                metadata: Metadata {
                    message: Some(err.to_string())
                }
            }), StatusCode::NOT_FOUND))
        }

        return Ok((web::Json(ApiResponse::<Option<Account>, Metadata> {
            is_array: false,
            data: None,
            metadata: Metadata {
                message: Some(err.to_string())
            }
        }), StatusCode::BAD_REQUEST))
    }

    Ok((web::Json(ApiResponse::<Option<Account>, Metadata> {
        is_array: false,
        data: account.ok(),
        metadata: Metadata {
            message: None
        }
    }), StatusCode::OK))
}