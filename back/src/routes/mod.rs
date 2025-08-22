use actix_web::web;

pub mod health;

pub fn entry(cfg: &mut web::ServiceConfig) {
	cfg.service(web::scope("/v1").configure(health::config));
}
