use actix_web::{
	App, HttpServer,
	middleware::{Logger, NormalizePath},
	web,
};
use dotenvy::dotenv;
use std::env;

use liga_muertos_back::{AppState, init_db, routes, utils::logging};

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	// Load environment variables from .env file
	dotenv().ok();

	// Get port from environment or use 4000 as default
	let port = env::var("PORT").unwrap_or("4000".to_owned());
	let port = port.parse::<u16>().expect("Invalid PORT value");

	// Configure enhanced logging
	logging::init();

	logging::startup_info(port);

	// Initialize database connection - fail fast if connection fails
	init_db().await?;

	// Start HTTP server
	logging::server_ready(port);

	HttpServer::new(|| {
		App::new()
			.wrap(Logger::default())
			.wrap(NormalizePath::trim())
			.app_data(web::Data::new(AppState::new()))
			.configure(routes::entry)
	})
	.bind(("0.0.0.0", port))?
	.run()
	.await?;

	Ok(())
}

#[cfg(test)]
mod tests {
	use super::*;
	use actix_web::{App, test, web};

	#[actix_web::test]
	async fn test_health_endpoint() {
		let app = test::init_service(
			App::new()
				.app_data(web::Data::new(AppState::new_test()))
				.configure(routes::entry),
		)
		.await;

		let req = test::TestRequest::get().uri("/v1/health").to_request();

		let resp = test::call_service(&app, req).await;
		assert!(resp.status().is_success());

		let body: serde_json::Value = test::read_body_json(resp).await;
		assert_eq!(body["name"], "La Liga de los Muertos");
		assert_eq!(body["status"], "OK");
	}
}
