use actix_web::web;

pub mod list;
pub mod create;
pub mod detail;
pub mod update;
pub mod delete;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(list::list_accounts);
    cfg.service(create::create_account);
    cfg.service(detail::detail_account);
    cfg.service(update::update_account);
    cfg.service(delete::delete_account);
}