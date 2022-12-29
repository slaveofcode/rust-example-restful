use actix_web::{delete, web, Result, Responder, http::StatusCode};
use chrono::Utc;
use serde::{Serialize, Deserialize};
use diesel::{prelude::*, result::Error};
use crate::{
    AppState,
    db::account::{Account, AccountDelete},
    controllers::{get_conn, ApiResponse, default_false},
    schema::accounts::dsl::{accounts, id, deleted_at},
};

#[derive(Serialize)]
struct Metadata {
    message: Option<String>
}

#[derive(Deserialize)]
struct AccountDeleteParam {
    #[serde(default = "default_false")]
    force: bool,
}

#[delete("/{account_id}")]
async fn delete_account(delete_data: web::Query<AccountDeleteParam>, path_param: web::Path<(i32,)>, app_state: web::Data<AppState>) -> Result<impl Responder> {
    let conn = &mut get_conn(&app_state);
    let account = accounts.filter(id.eq(path_param.0)).first::<Account>(conn);

    // if account already removed
    if let Err(msg @ Error::NotFound) = account {
        return Ok((web::Json(ApiResponse::<Option<Account>, Metadata> {
            is_array: false,
            data: None,
            metadata: Metadata {
                message: Some(msg.to_string())
            }
        }), StatusCode::NO_CONTENT));
    }

    if let Err(msg) = account {
        return Ok((web::Json(ApiResponse::<Option<Account>, Metadata> {
            is_array: false,
            data: None,
            metadata: Metadata {
                message: Some(msg.to_string())
            }
        }), StatusCode::BAD_REQUEST))
    }

    if let Ok(acc) = account {
        // if the force flag is true
        // will hardly delete the record
        if delete_data.force {
            let deleted_account = diesel::delete(
                accounts.filter(id.eq(path_param.0)),
            ).execute(conn);

            return match deleted_account {
                Err(msg) => Ok((web::Json(ApiResponse::<Option<Account>, Metadata> {
                    is_array: false,
                    data: None,
                    metadata: Metadata {
                        message: Some(msg.to_string())
                    }
                }), StatusCode::BAD_REQUEST)),
                _ => Ok((web::Json(ApiResponse::<Option<Account>, Metadata> {
                    is_array: false,
                    data: None,
                    metadata: Metadata {
                        message: None
                    }
                }), StatusCode::NO_CONTENT))
            }
        }

        // otherwise check the record is flagged as deleted (deleted_at)
        if !acc.deleted_at.is_none() {
            return Ok((web::Json(ApiResponse::<Option<Account>, Metadata> {
                is_array: false,
                data: None,
                metadata: Metadata {
                    message: None
                }
            }), StatusCode::NO_CONTENT));
        }
    }

    let filter_undeleted = accounts
        .filter(id.eq(path_param.0))
        .filter(deleted_at.is_null());

    // if the steps got here
    // do soft delete (deleted_at)
    let deleted_account = diesel::update(filter_undeleted)
        .set::<AccountDelete>(AccountDelete {
            updated_at: Utc::now().naive_utc(),
            deleted_at: Utc::now().naive_utc(),
        })
        .get_result::<Account>(conn);

    if let Err(msg) = deleted_account {
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
        data: None,
        metadata: Metadata {
            message: None
        }
    }), StatusCode::NO_CONTENT))
}