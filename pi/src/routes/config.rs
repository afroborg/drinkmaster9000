use actix_web::{web, Scope};

pub fn config_scope() -> Scope {
    web::scope("/config")
}
