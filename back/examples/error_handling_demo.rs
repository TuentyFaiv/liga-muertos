//! Error Handling Demo Example
//!
//! This example demonstrates the comprehensive error handling system
//! for API endpoints in the Liga de los Muertos backend.
//!
//! Run this example with:
//! ```bash
//! cargo run --example error_handling_demo
//! ```

use actix_web::{App, HttpResponse, HttpServer, Result as ActixResult, web};
use liga_muertos_back::utils::{
	error::{
		ApiError, ApiResult,
		validation::{ValidationBuilder, validators},
	},
	logging,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserRequest {
	pub username: String,
	pub email: String,
	pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
	pub id: String,
	pub username: String,
	pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TournamentRequest {
	pub name: String,
	pub description: String,
	pub max_participants: i32,
}

/// Example endpoint demonstrating validation errors
async fn create_user(req: web::Json<CreateUserRequest>) -> ActixResult<HttpResponse, ApiError> {
	logging::request_debug("POST", "/api/users", None);

	// Validate input using the validation system
	let validation_result = ValidationBuilder::new()
		.validate(|| validators::required(Some(&req.username), "username").map(|_| ()))
		.validate(|| validators::username(&req.username, "username"))
		.validate(|| validators::email(&req.email, "email"))
		.validate(|| validators::password(&req.password, "password"))
		.build_unit();

	if let Err(validation_errors) = validation_result {
		// Convert validation errors to API error
		if let Some(first_error) = validation_errors.first() {
			return Err(ApiError::validation_with_field(
				&first_error.message,
				first_error.field.as_deref().unwrap_or("unknown"),
			));
		}
		return Err(ApiError::validation("Multiple validation errors occurred"));
	}

	// Simulate database check for existing user
	if req.username == "admin" {
		return Err(ApiError::conflict("Username already exists"));
	}

	// Simulate database operation that might fail
	if req.username == "error" {
		return Err(ApiError::Database {
			message: "Failed to create user in database".to_string(),
		});
	}

	// Success case
	let user = UserResponse {
		id: format!("user_{}", uuid::Uuid::new_v4().simple()),
		username: req.username.clone(),
		email: req.email.clone(),
	};

	logging::auth_event("user_created", Some(&user.id));

	Ok(HttpResponse::Created().json(user))
}

/// Example endpoint demonstrating not found errors
async fn get_user(path: web::Path<String>) -> ActixResult<HttpResponse, ApiError> {
	let user_id = path.into_inner();
	logging::request_debug("GET", &format!("/api/users/{}", user_id), None);

	// Simulate user lookup
	if user_id == "nonexistent" {
		return Err(ApiError::not_found("user", &user_id));
	}

	if user_id == "error" {
		return Err(ApiError::Database {
			message: "Database query failed".to_string(),
		});
	}

	// Success case
	let user = UserResponse {
		id: user_id.clone(),
		username: "example_user".to_string(),
		email: "user@example.com".to_string(),
	};

	Ok(HttpResponse::Ok().json(user))
}

/// Example endpoint demonstrating authorization errors
async fn delete_user(path: web::Path<String>) -> ActixResult<HttpResponse, ApiError> {
	let user_id = path.into_inner();
	logging::request_debug("DELETE", &format!("/api/users/{}", user_id), None);

	// Simulate authorization check
	if user_id == "unauthorized" {
		return Err(ApiError::authorization(
			"You don't have permission to delete this user",
		));
	}

	if user_id == "protected" {
		return Err(ApiError::authorization("Cannot delete protected user"));
	}

	// Success case
	logging::auth_event("user_deleted", Some(&user_id));
	Ok(HttpResponse::NoContent().finish())
}

/// Example endpoint demonstrating tournament-specific errors
async fn create_tournament(
	req: web::Json<TournamentRequest>,
) -> ActixResult<HttpResponse, ApiError> {
	logging::request_debug("POST", "/api/tournaments", None);

	// Validate tournament data
	let validation_result = ValidationBuilder::new()
		.validate(|| validators::tournament_name(&req.name, "name"))
		.validate(|| validators::length(&req.description, 10, 500, "description"))
		.validate(|| validators::range(req.max_participants, 2, 256, "max_participants"))
		.build_unit();

	if let Err(validation_errors) = validation_result {
		if let Some(first_error) = validation_errors.first() {
			return Err(ApiError::validation_with_field(
				&first_error.message,
				first_error.field.as_deref().unwrap_or("unknown"),
			));
		}
	}

	// Simulate tournament business logic errors
	if req.name.to_lowercase().contains("forbidden") {
		return Err(ApiError::tournament(
			"Tournament name contains forbidden words",
		));
	}

	if req.max_participants > 128 {
		return Err(ApiError::tournament_with_id(
			"Tournament too large for current infrastructure",
			"temp_tournament_id",
		));
	}

	// Success case
	let tournament_id = format!("tournament_{}", uuid::Uuid::new_v4().simple());
	logging::tournament_event("created", &tournament_id, Some("demo_user"));

	Ok(HttpResponse::Created().json(serde_json::json!({
			"id": tournament_id,
			"name": req.name,
			"description": req.description,
			"max_participants": req.max_participants,
			"status": "created"
	})))
}

/// Example endpoint demonstrating external service errors
async fn sync_with_external() -> ActixResult<HttpResponse, ApiError> {
	logging::request_debug("POST", "/api/sync", None);

	// Simulate external service call
	let should_fail = std::env::var("DEMO_EXTERNAL_FAIL").unwrap_or("false".to_string()) == "true";

	if should_fail {
		return Err(ApiError::ExternalService {
			service: "Steam API".to_string(),
			message: "Failed to sync player data".to_string(),
		});
	}

	Ok(HttpResponse::Ok().json(serde_json::json!({
			"message": "Sync completed successfully",
			"synced_players": 42
	})))
}

/// Example endpoint demonstrating internal server errors
async fn simulate_internal_error() -> ActixResult<HttpResponse, ApiError> {
	logging::request_debug("GET", "/api/internal-error", None);

	// Simulate an unexpected internal error
	Err(ApiError::internal("Unexpected error during processing"))
}

/// Example endpoint demonstrating rate limiting
async fn rate_limited_endpoint() -> ActixResult<HttpResponse, ApiError> {
	logging::request_debug("GET", "/api/rate-limited", None);

	// Simulate rate limiting
	let should_rate_limit = std::env::var("DEMO_RATE_LIMIT").unwrap_or("false".to_string()) == "true";

	if should_rate_limit {
		return Err(ApiError::RateLimit {
			message: "Too many requests. Please try again in 60 seconds".to_string(),
		});
	}

	Ok(HttpResponse::Ok().json(serde_json::json!({
			"message": "Request processed successfully",
			"remaining_requests": 99
	})))
}

/// Demonstration of error handling with ApiResult helper
async fn complex_operation() -> ApiResult<String> {
	// Simulate multiple operations that can fail
	validate_permissions()?;
	let data = fetch_data().await?;
	process_data(&data)?;

	Ok("Operation completed successfully".to_string())
}

fn validate_permissions() -> ApiResult<()> {
	// Simulate permission check
	Ok(())
}

async fn fetch_data() -> ApiResult<String> {
	// Simulate data fetching
	Ok("sample_data".to_string())
}

fn process_data(_data: &str) -> ApiResult<()> {
	// Simulate data processing
	Ok(())
}

/// Health check endpoint (should always work)
async fn health_check() -> ActixResult<HttpResponse, ApiError> {
	Ok(HttpResponse::Ok().json(serde_json::json!({
			"status": "healthy",
			"message": "Error handling demo is running"
	})))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	// Initialize logging
	logging::init();
	logging::startup_info(8080);

	println!("\n🎯 Error Handling Demo Server");
	println!("================================");
	println!("Try these endpoints to see different error types:");
	println!("");
	println!("✅ Success cases:");
	println!("  curl -X GET http://localhost:8080/health");
	println!("  curl -X GET http://localhost:8080/users/valid_user");
	println!("  curl -X POST http://localhost:8080/users \\");
	println!("    -H 'Content-Type: application/json' \\");
	println!(
		"    -d '{{\"username\":\"testuser\",\"email\":\"test@example.com\",\"password\":\"Password123\"}}'"
	);
	println!("");
	println!("❌ Error cases:");
	println!("  curl -X GET http://localhost:8080/users/nonexistent    # 404 Not Found");
	println!("  curl -X DELETE http://localhost:8080/users/unauthorized # 403 Forbidden");
	println!("  curl -X POST http://localhost:8080/users \\");
	println!("    -H 'Content-Type: application/json' \\");
	println!(
		"    -d '{{\"username\":\"a\",\"email\":\"invalid\",\"password\":\"weak\"}}'  # 400 Validation Error"
	);
	println!("  curl -X POST http://localhost:8080/tournaments \\");
	println!("    -H 'Content-Type: application/json' \\");
	println!(
		"    -d '{{\"name\":\"forbidden tournament\",\"description\":\"test\",\"max_participants\":300}}'"
	);
	println!("");
	println!("🔧 Environment variables:");
	println!("  DEMO_EXTERNAL_FAIL=true   # Makes /sync endpoint fail");
	println!("  DEMO_RATE_LIMIT=true      # Makes /rate-limited endpoint fail");
	println!("");

	HttpServer::new(|| {
		App::new()
			.route("/health", web::get().to(health_check))
			.route("/users", web::post().to(create_user))
			.route("/users/{id}", web::get().to(get_user))
			.route("/users/{id}", web::delete().to(delete_user))
			.route("/tournaments", web::post().to(create_tournament))
			.route("/sync", web::post().to(sync_with_external))
			.route("/internal-error", web::get().to(simulate_internal_error))
			.route("/rate-limited", web::get().to(rate_limited_endpoint))
	})
	.bind("127.0.0.1:8080")?
	.run()
	.await
}

// Simple UUID implementation for the demo
mod uuid {
	pub struct Uuid;
	impl Uuid {
		pub fn new_v4() -> Self {
			Self
		}
		pub fn simple(&self) -> String {
			format!(
				"{:08x}{:04x}{:04x}{:04x}{:012x}",
				rand::random::<u32>(),
				rand::random::<u16>(),
				rand::random::<u16>(),
				rand::random::<u16>(),
				rand::random::<u64>() & 0xffffffffffff
			)
		}
	}

	mod rand {
		pub fn random<T: Default>() -> T {
			T::default()
		}
	}
}
