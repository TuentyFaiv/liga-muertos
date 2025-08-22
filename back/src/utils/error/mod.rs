//! Error handling module for the Liga de los Muertos backend
//!
//! This module provides comprehensive error handling for all API endpoints,
//! with proper HTTP status codes, structured error responses, and logging.

use actix_web::{HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};

use thiserror::Error;

pub mod validation;

/// Standard API error response format
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiErrorResponse {
	/// Whether the request was successful (always false for errors)
	pub success: bool,
	/// Human-readable error message
	pub message: String,
	/// Error code for programmatic handling
	pub error_code: String,
	/// Additional error details (optional)
	pub details: Option<serde_json::Value>,
	/// Request ID for tracing (optional)
	pub request_id: Option<String>,
}

impl ApiErrorResponse {
	/// Create a new error response
	pub fn new(message: String, error_code: String) -> Self {
		Self {
			success: false,
			message,
			error_code,
			details: None,
			request_id: None,
		}
	}

	/// Add additional details to the error response
	pub fn with_details(mut self, details: serde_json::Value) -> Self {
		self.details = Some(details);
		self
	}

	/// Add request ID for tracing
	pub fn with_request_id(mut self, request_id: String) -> Self {
		self.request_id = Some(request_id);
		self
	}
}

/// Main error enum for the application
#[derive(Error, Debug)]
pub enum ApiError {
	/// Database-related errors
	#[error("Database error: {message}")]
	Database { message: String },

	/// Validation errors
	#[error("Validation error: {message}")]
	Validation {
		message: String,
		field: Option<String>,
	},

	/// Authentication errors
	#[error("Authentication error: {message}")]
	Authentication { message: String },

	/// Authorization errors
	#[error("Authorization error: {message}")]
	Authorization { message: String },

	/// Resource not found errors
	#[error("Resource not found: {resource} with id {id}")]
	NotFound { resource: String, id: String },

	/// Conflict errors (e.g., duplicate resources)
	#[error("Conflict: {message}")]
	Conflict { message: String },

	/// Rate limiting errors
	#[error("Rate limit exceeded: {message}")]
	RateLimit { message: String },

	/// External service errors
	#[error("External service error: {service} - {message}")]
	ExternalService { service: String, message: String },

	/// Internal server errors
	#[error("Internal server error: {message}")]
	Internal { message: String },

	/// Bad request errors
	#[error("Bad request: {message}")]
	BadRequest { message: String },

	/// JSON parsing errors
	#[error("JSON parsing error: {message}")]
	JsonParsing { message: String },

	/// Tournament-specific errors
	#[error("Tournament error: {message}")]
	Tournament {
		message: String,
		tournament_id: Option<String>,
	},

	/// User-specific errors
	#[error("User error: {message}")]
	User {
		message: String,
		user_id: Option<String>,
	},
}

impl ApiError {
	/// Get the HTTP status code for this error
	pub fn status_code(&self) -> actix_web::http::StatusCode {
		use actix_web::http::StatusCode;

		match self {
			ApiError::Database { .. } => StatusCode::INTERNAL_SERVER_ERROR,
			ApiError::Validation { .. } => StatusCode::BAD_REQUEST,
			ApiError::Authentication { .. } => StatusCode::UNAUTHORIZED,
			ApiError::Authorization { .. } => StatusCode::FORBIDDEN,
			ApiError::NotFound { .. } => StatusCode::NOT_FOUND,
			ApiError::Conflict { .. } => StatusCode::CONFLICT,
			ApiError::RateLimit { .. } => StatusCode::TOO_MANY_REQUESTS,
			ApiError::ExternalService { .. } => StatusCode::SERVICE_UNAVAILABLE,
			ApiError::Internal { .. } => StatusCode::INTERNAL_SERVER_ERROR,
			ApiError::BadRequest { .. } => StatusCode::BAD_REQUEST,
			ApiError::JsonParsing { .. } => StatusCode::BAD_REQUEST,
			ApiError::Tournament { .. } => StatusCode::BAD_REQUEST,
			ApiError::User { .. } => StatusCode::BAD_REQUEST,
		}
	}

	/// Get the error code for this error
	pub fn error_code(&self) -> String {
		match self {
			ApiError::Database { .. } => "DATABASE_ERROR".to_string(),
			ApiError::Validation { .. } => "VALIDATION_ERROR".to_string(),
			ApiError::Authentication { .. } => "AUTHENTICATION_ERROR".to_string(),
			ApiError::Authorization { .. } => "AUTHORIZATION_ERROR".to_string(),
			ApiError::NotFound { .. } => "NOT_FOUND".to_string(),
			ApiError::Conflict { .. } => "CONFLICT".to_string(),
			ApiError::RateLimit { .. } => "RATE_LIMIT_EXCEEDED".to_string(),
			ApiError::ExternalService { .. } => "EXTERNAL_SERVICE_ERROR".to_string(),
			ApiError::Internal { .. } => "INTERNAL_SERVER_ERROR".to_string(),
			ApiError::BadRequest { .. } => "BAD_REQUEST".to_string(),
			ApiError::JsonParsing { .. } => "JSON_PARSING_ERROR".to_string(),
			ApiError::Tournament { .. } => "TOURNAMENT_ERROR".to_string(),
			ApiError::User { .. } => "USER_ERROR".to_string(),
		}
	}

	/// Create additional details for the error response
	pub fn details(&self) -> Option<serde_json::Value> {
		match self {
			ApiError::Validation { field, .. } => {
				field.as_ref().map(|f| serde_json::json!({ "field": f }))
			}
			ApiError::NotFound { resource, id } => Some(serde_json::json!({
					"resource": resource,
					"id": id
			})),
			ApiError::ExternalService { service, .. } => Some(serde_json::json!({ "service": service })),
			ApiError::Tournament { tournament_id, .. } => tournament_id
				.as_ref()
				.map(|id| serde_json::json!({ "tournament_id": id })),
			ApiError::User { user_id, .. } => user_id
				.as_ref()
				.map(|id| serde_json::json!({ "user_id": id })),
			_ => None,
		}
	}

	/// Check if this error should be logged as an error (vs warning/info)
	pub fn should_log_as_error(&self) -> bool {
		matches!(
			self,
			ApiError::Database { .. } | ApiError::Internal { .. } | ApiError::ExternalService { .. }
		)
	}
}

impl ResponseError for ApiError {
	fn error_response(&self) -> HttpResponse {
		// Log the error with appropriate level
		if self.should_log_as_error() {
			log::error!("API Error: {self}");
		} else {
			log::warn!("API Warning: {self}");
		}

		let mut error_response = ApiErrorResponse::new(self.to_string(), self.error_code());

		if let Some(details) = self.details() {
			error_response = error_response.with_details(details);
		}

		HttpResponse::build(self.status_code()).json(error_response)
	}

	fn status_code(&self) -> actix_web::http::StatusCode {
		self.status_code()
	}
}

/// Conversion from SurrealDB errors
impl From<surrealdb::Error> for ApiError {
	fn from(error: surrealdb::Error) -> Self {
		log::error!("SurrealDB error: {error}");

		// Try to categorize SurrealDB errors
		let error_string = error.to_string();

		if error_string.contains("Connection") || error_string.contains("timeout") {
			ApiError::Database {
				message: "Database connection error".to_string(),
			}
		} else if error_string.contains("duplicate") || error_string.contains("already exists") {
			ApiError::Conflict {
				message: "Resource already exists".to_string(),
			}
		} else if error_string.contains("not found") || error_string.contains("No record") {
			ApiError::NotFound {
				resource: "record".to_string(),
				id: "unknown".to_string(),
			}
		} else {
			ApiError::Database {
				message: error.to_string(),
			}
		}
	}
}

/// Conversion from JSON parsing errors
impl From<serde_json::Error> for ApiError {
	fn from(error: serde_json::Error) -> Self {
		log::warn!("JSON parsing error: {error}");
		ApiError::JsonParsing {
			message: error.to_string(),
		}
	}
}

/// Conversion from validation errors
impl From<validation::ValidationError> for ApiError {
	fn from(error: validation::ValidationError) -> Self {
		ApiError::Validation {
			message: error.message.clone(),
			field: error.field.clone(),
		}
	}
}

/// Helper type alias for API results
pub type ApiResult<T> = Result<T, ApiError>;

/// Extension trait for results to add context
pub trait ApiResultExt<T> {
	/// Add context to an error
	fn with_context(self, context: &str) -> ApiResult<T>;

	/// Convert to a not found error
	fn not_found(self, resource: &str, id: &str) -> ApiResult<T>;

	/// Convert to a validation error
	fn validation_error(self, field: &str) -> ApiResult<T>;
}

impl<T, E> ApiResultExt<T> for Result<T, E>
where
	E: std::error::Error,
{
	fn with_context(self, context: &str) -> ApiResult<T> {
		self.map_err(|e| ApiError::Internal {
			message: format!("{context}: {e}"),
		})
	}

	fn not_found(self, resource: &str, id: &str) -> ApiResult<T> {
		self.map_err(|_| ApiError::NotFound {
			resource: resource.to_string(),
			id: id.to_string(),
		})
	}

	fn validation_error(self, field: &str) -> ApiResult<T> {
		self.map_err(|e| ApiError::Validation {
			message: e.to_string(),
			field: Some(field.to_string()),
		})
	}
}

/// Convenience functions for creating specific errors
impl ApiError {
	/// Create a validation error
	pub fn validation(message: &str) -> Self {
		ApiError::Validation {
			message: message.to_string(),
			field: None,
		}
	}

	/// Create a validation error with field
	pub fn validation_with_field(message: &str, field: &str) -> Self {
		ApiError::Validation {
			message: message.to_string(),
			field: Some(field.to_string()),
		}
	}

	/// Create a not found error
	pub fn not_found(resource: &str, id: &str) -> Self {
		ApiError::NotFound {
			resource: resource.to_string(),
			id: id.to_string(),
		}
	}

	/// Create an authentication error
	pub fn authentication(message: &str) -> Self {
		ApiError::Authentication {
			message: message.to_string(),
		}
	}

	/// Create an authorization error
	pub fn authorization(message: &str) -> Self {
		ApiError::Authorization {
			message: message.to_string(),
		}
	}

	/// Create a conflict error
	pub fn conflict(message: &str) -> Self {
		ApiError::Conflict {
			message: message.to_string(),
		}
	}

	/// Create a bad request error
	pub fn bad_request(message: &str) -> Self {
		ApiError::BadRequest {
			message: message.to_string(),
		}
	}

	/// Create an internal error
	pub fn internal(message: &str) -> Self {
		ApiError::Internal {
			message: message.to_string(),
		}
	}

	/// Create a tournament error
	pub fn tournament(message: &str) -> Self {
		ApiError::Tournament {
			message: message.to_string(),
			tournament_id: None,
		}
	}

	/// Create a tournament error with ID
	pub fn tournament_with_id(message: &str, tournament_id: &str) -> Self {
		ApiError::Tournament {
			message: message.to_string(),
			tournament_id: Some(tournament_id.to_string()),
		}
	}

	/// Create a user error
	pub fn user(message: &str) -> Self {
		ApiError::User {
			message: message.to_string(),
			user_id: None,
		}
	}

	/// Create a user error with ID
	pub fn user_with_id(message: &str, user_id: &str) -> Self {
		ApiError::User {
			message: message.to_string(),
			user_id: Some(user_id.to_string()),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use actix_web::http::StatusCode;

	#[test]
	fn test_api_error_status_codes() {
		assert_eq!(
			ApiError::validation("test").status_code(),
			StatusCode::BAD_REQUEST
		);
		assert_eq!(
			ApiError::not_found("user", "123").status_code(),
			StatusCode::NOT_FOUND
		);
		assert_eq!(
			ApiError::authentication("invalid token").status_code(),
			StatusCode::UNAUTHORIZED
		);
		assert_eq!(
			ApiError::authorization("insufficient permissions").status_code(),
			StatusCode::FORBIDDEN
		);
	}

	#[test]
	fn test_api_error_codes() {
		assert_eq!(
			ApiError::validation("test").error_code(),
			"VALIDATION_ERROR"
		);
		assert_eq!(ApiError::not_found("user", "123").error_code(), "NOT_FOUND");
		assert_eq!(
			ApiError::internal("test").error_code(),
			"INTERNAL_SERVER_ERROR"
		);
	}

	#[test]
	fn test_api_error_response_creation() {
		let response = ApiErrorResponse::new("Test error".to_string(), "TEST_ERROR".to_string());

		assert!(!response.success);
		assert_eq!(response.message, "Test error");
		assert_eq!(response.error_code, "TEST_ERROR");
		assert!(response.details.is_none());
		assert!(response.request_id.is_none());
	}

	#[test]
	fn test_error_details() {
		let error = ApiError::NotFound {
			resource: "tournament".to_string(),
			id: "123".to_string(),
		};

		let details = error.details().unwrap();
		assert_eq!(details["resource"], "tournament");
		assert_eq!(details["id"], "123");
	}

	#[test]
	fn test_should_log_as_error() {
		assert!(
			ApiError::Database {
				message: "test".to_string()
			}
			.should_log_as_error()
		);
		assert!(
			ApiError::Internal {
				message: "test".to_string()
			}
			.should_log_as_error()
		);
		assert!(
			!ApiError::Validation {
				message: "test".to_string(),
				field: None
			}
			.should_log_as_error()
		);
		assert!(
			!ApiError::NotFound {
				resource: "test".to_string(),
				id: "123".to_string()
			}
			.should_log_as_error()
		);
	}
}
