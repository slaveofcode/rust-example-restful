use actix_web::{get, web, Result, Responder};
use serde::Serialize;
use crate::{AppState};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(ping);
    // cfg.route("", web::get().to(HttpResponse::InternalServerError));
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct PingResp {
    app: String,
    status: String,
}

#[get("ping")]
async fn ping(data: web::Data<AppState>) -> Result<impl Responder> {
    let resp = PingResp {
        app: data.app_name.to_string(),
        status: match data.is_maintenance {
            true => "on maintenance".to_owned(),
            _ => "alive".to_owned(),
        }
    };
    Ok(web::Json(resp))
}