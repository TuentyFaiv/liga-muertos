//! Entity definitions for the Liga de los Muertos backend
//!
//! These structs represent the data models used throughout the application
//! and correspond to the database tables defined in the schema.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub mod participant;
pub mod tournament;
pub mod user;

pub use participant::*;
pub use tournament::*;
pub use user::*;

/// Common fields that appear in many entities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Timestamps {
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
}

/// Standard response wrapper for API endpoints
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
	pub success: bool,
	pub data: Option<T>,
	pub message: Option<String>,
	pub errors: Option<Vec<String>>,
}

impl<T> ApiResponse<T> {
	pub fn success(data: T) -> Self {
		Self {
			success: true,
			data: Some(data),
			message: None,
			errors: None,
		}
	}

	pub fn success_with_message(data: T, message: String) -> Self {
		Self {
			success: true,
			data: Some(data),
			message: Some(message),
			errors: None,
		}
	}

	pub fn error(message: String) -> Self {
		Self {
			success: false,
			data: None,
			message: Some(message),
			errors: None,
		}
	}

	pub fn errors(errors: Vec<String>) -> Self {
		Self {
			success: false,
			data: None,
			message: None,
			errors: Some(errors),
		}
	}
}
