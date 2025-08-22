use crate::DB;
use crate::utils::error::ApiResult;
use actix_web::{HttpResponse, get, web};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct HealthStatus {
	pub name: String,
	pub status: String,
	pub version: String,
	pub database: Option<String>,
}

#[get("")]
async fn status() -> ApiResult<HttpResponse> {
	// Check database connection by trying a simple query
	let db_status = match DB.query("RETURN 'connected'").await {
		Ok(_) => Some("Connected".to_string()),
		Err(e) => {
			// Log database connection issue but don't fail health check
			log::warn!("Health check database query failed: {}", e);
			Some("Disconnected".to_string())
		}
	};

	let health = HealthStatus {
		name: "La Liga de los Muertos".to_string(),
		status: "OK".to_string(),
		version: "v0.1.0".to_string(),
		database: db_status,
	};

	Ok(HttpResponse::Ok().json(health))
}

pub fn config(cfg: &mut web::ServiceConfig) {
	cfg.service(web::scope("/health").service(status));
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::AppState;
	use actix_web::{App, test};

	#[actix_web::test]
	async fn test_health_endpoint() {
		// Use the test app state
		let app = test::init_service(
			App::new()
				.app_data(web::Data::new(AppState::new_test()))
				.configure(config),
		)
		.await;

		let req = test::TestRequest::get().uri("/health").to_request();

		let resp = test::call_service(&app, req).await;
		assert!(resp.status().is_success());

		let body: HealthStatus = test::read_body_json(resp).await;
		assert_eq!(body.name, "La Liga de los Muertos");
		assert_eq!(body.status, "OK");
	}

	// Use actix_web test macro for consistency
	#[actix_web::test]
	async fn test_health_status_creation() {
		let health = HealthStatus {
			name: "Test".to_string(),
			status: "OK".to_string(),
			version: "v1.0.0".to_string(),
			database: Some("Connected".to_string()),
		};

		assert_eq!(health.name, "Test");
		assert_eq!(health.status, "OK");
		assert_eq!(health.version, "v1.0.0");
		assert_eq!(health.database, Some("Connected".to_string()));
	}
}
