mod config;
mod schema;
mod db;
mod controllers;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer, middleware, http::{header}, error::{self}, HttpResponse};
use controllers::ApiResponse;
use db::connector::{get_pool, PgPool};
use serde::Serialize;

struct AppState {
    app_name: String,
    is_maintenance: bool,
    db_pool: PgPool,
}

#[derive(Serialize)]
struct AppResponseError {
    message: String
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_config = config::app::Config::from_env().unwrap();
    let pool_connections = get_pool(
        app_config.db_url,
        app_config.db_max_conn.or(Some(1)),
    );

    let is_maintenance = match app_config.mode.as_str() {
        "maintenance" => true,
        _ => false,
    };

    let mut svr = HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&app_config.cors_origin)
            .allowed_methods(vec!["OPTIONS", "HEAD", "GET", "POST", "PUT", "PATCH", "DELETE"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .supports_credentials()
            .max_age(3600);

        let json_config = web::JsonConfig::default()
            // limit request payload size
            .limit(4096)
            // only accept text/plain content type
            .content_type(|mime| mime == mime::TEXT_PLAIN)
            .error_handler(|err, _req| {
                let err_message = err.to_string();
                let resp = ApiResponse::<(), AppResponseError> {
                    is_array: false,
                    data: (),
                    metadata: AppResponseError {
                        message: err_message,
                    }
                };

                error::InternalError::from_response(err, HttpResponse::BadRequest().json(resp)).into()
            });
    

        App::new()
            .app_data(json_config)
            .app_data(web::Data::new(AppState {
                is_maintenance,
                app_name: String::from("Restful API"),
                db_pool: pool_connections.clone(),
            }))
            .wrap(middleware::DefaultHeaders::new().add(("X-Api-Ver", "v1")))
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .wrap(cors)
            .service(web::scope("/").configure(controllers::main::routes))
            .service(web::scope("/accounts").configure(controllers::accounts::routes))

    });

    if app_config.mode == "development" {
        svr = svr.workers(1);
    }

    let start = match svr.bind((app_config.host, app_config.port)) {
        Ok(inst) => inst,
        Err(err) => panic!("Unable to start server: {err}"),
    };

    start.run().await
}
