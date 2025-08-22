//! Utility modules for the Liga de los Muertos backend
//!
//! This module contains various utility functions and helpers used throughout
//! the application to provide common functionality like logging, validation,
//! error handling, and other cross-cutting concerns.

pub mod error;
pub mod logging;

// Re-export modules for clean imports
// Usage: use liga_muertos_back::utils::logging;
//        logging::init();
//        logging::startup_info(port);

/// Application constants and configuration values
pub mod constants {
	/// Default port for the HTTP server
	pub const DEFAULT_PORT: u16 = 4000;

	/// Default database URL for SurrealDB
	pub const DEFAULT_DB_URL: &str = "ws://localhost:8000";

	/// Default namespace for SurrealDB
	pub const DEFAULT_NAMESPACE: &str = "liga";

	/// Default database name for SurrealDB
	pub const DEFAULT_DATABASE: &str = "muertos";

	/// Default username for SurrealDB
	pub const DEFAULT_USERNAME: &str = "root";

	/// Default password for SurrealDB
	pub const DEFAULT_PASSWORD: &str = "root";

	/// Application name
	pub const APP_NAME: &str = "La Liga de los Muertos";

	/// Application version
	pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

	/// API version prefix
	pub const API_VERSION: &str = "v1";

	/// Maximum request body size (in bytes) - 1MB
	pub const MAX_REQUEST_SIZE: usize = 1_048_576;

	/// Request timeout in seconds
	pub const REQUEST_TIMEOUT_SECONDS: u64 = 30;

	/// Database connection timeout in seconds
	pub const DB_CONNECTION_TIMEOUT_SECONDS: u64 = 10;

	/// JWT token expiration time in hours
	pub const JWT_EXPIRATION_HOURS: i64 = 24;
}

/// Validation utilities
pub mod validation {
	/// Check if an email address is valid (basic validation)
	pub fn is_valid_email(email: &str) -> bool {
		email.contains('@')
			&& email.len() > 3
			&& email.len() < 255
			&& !email.starts_with('@')
			&& !email.ends_with('@')
			&& email.chars().all(|c| c.is_ascii())
	}

	/// Check if a username is valid
	pub fn is_valid_username(username: &str) -> bool {
		const MIN_LENGTH: usize = 3;
		const MAX_LENGTH: usize = 50;

		if username.len() < MIN_LENGTH || username.len() > MAX_LENGTH {
			return false;
		}

		// Username should start with alphanumeric character
		if !username
			.chars()
			.next()
			.map(|c| c.is_alphanumeric())
			.unwrap_or(false)
		{
			return false;
		}

		// Username should contain only alphanumeric characters, hyphens, and underscores
		username
			.chars()
			.all(|c| c.is_alphanumeric() || c == '-' || c == '_')
	}

	/// Check if a tournament name is valid
	pub fn is_valid_tournament_name(name: &str) -> bool {
		const MIN_LENGTH: usize = 3;
		const MAX_LENGTH: usize = 100;

		if name.is_empty() || name.len() < MIN_LENGTH || name.len() > MAX_LENGTH {
			return false;
		}

		// Tournament name should not be only whitespace
		!name.trim().is_empty()
	}

	/// Check if a password meets minimum requirements
	pub fn is_valid_password(password: &str) -> bool {
		const MIN_LENGTH: usize = 8;
		const MAX_LENGTH: usize = 128;

		if password.len() < MIN_LENGTH || password.len() > MAX_LENGTH {
			return false;
		}

		let has_lowercase = password.chars().any(|c| c.is_lowercase());
		let has_uppercase = password.chars().any(|c| c.is_uppercase());
		let has_digit = password.chars().any(|c| c.is_ascii_digit());

		has_lowercase && has_uppercase && has_digit
	}

	/// Validate that required fields are present in a collection
	pub fn validate_required_fields(fields: &[(&str, Option<&str>)]) -> Result<(), Vec<String>> {
		let missing_fields: Vec<String> = fields
			.iter()
			.filter_map(|(field_name, value)| match value {
				Some(v) if !v.trim().is_empty() => None,
				_ => Some(field_name.to_string()),
			})
			.collect();

		if missing_fields.is_empty() {
			Ok(())
		} else {
			Err(missing_fields)
		}
	}

	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_email_validation() {
			assert!(is_valid_email("user@example.com"));
			assert!(is_valid_email("test.user+tag@domain.co.uk"));
			assert!(!is_valid_email("invalid-email"));
			assert!(!is_valid_email("@example.com"));
			assert!(!is_valid_email("user@"));
			assert!(!is_valid_email(""));
		}

		#[test]
		fn test_username_validation() {
			assert!(is_valid_username("user123"));
			assert!(is_valid_username("test-user"));
			assert!(is_valid_username("user_name"));
			assert!(!is_valid_username("us")); // too short
			assert!(!is_valid_username("-user")); // starts with hyphen
			assert!(!is_valid_username("user@name")); // invalid character
		}

		#[test]
		fn test_tournament_name_validation() {
			assert!(is_valid_tournament_name("My Tournament"));
			assert!(is_valid_tournament_name("Spring Championship 2024"));
			assert!(!is_valid_tournament_name(""));
			assert!(!is_valid_tournament_name("  "));
			assert!(!is_valid_tournament_name("ab")); // too short
		}

		#[test]
		fn test_password_validation() {
			assert!(is_valid_password("Password123"));
			assert!(is_valid_password("MyStr0ngP@ssw0rd"));
			assert!(!is_valid_password("password")); // no uppercase or digit
			assert!(!is_valid_password("PASSWORD123")); // no lowercase
			assert!(!is_valid_password("Password")); // no digit
			assert!(!is_valid_password("Pass1")); // too short
		}

		#[test]
		fn test_required_fields_validation() {
			let fields = vec![
				("username", Some("john")),
				("email", Some("john@example.com")),
				("password", Some("Password123")),
			];
			assert!(validate_required_fields(&fields).is_ok());

			let fields_with_missing = vec![
				("username", Some("john")),
				("email", None),
				("password", Some("")),
			];
			let result = validate_required_fields(&fields_with_missing);
			assert!(result.is_err());
			let errors = result.unwrap_err();
			assert_eq!(errors.len(), 2);
			assert!(errors.contains(&"email".to_string()));
			assert!(errors.contains(&"password".to_string()));
		}
	}
}

/// Time and date utilities
pub mod time {
	use chrono::{DateTime, Duration, Utc};

	/// Get current UTC timestamp
	pub fn now_utc() -> DateTime<Utc> {
		Utc::now()
	}

	/// Get a timestamp N hours from now
	pub fn hours_from_now(hours: i64) -> DateTime<Utc> {
		Utc::now() + Duration::hours(hours)
	}

	/// Get a timestamp N days from now
	pub fn days_from_now(days: i64) -> DateTime<Utc> {
		Utc::now() + Duration::days(days)
	}

	/// Format a timestamp for display
	pub fn format_timestamp(timestamp: &DateTime<Utc>) -> String {
		timestamp.format("%Y-%m-%d %H:%M:%S UTC").to_string()
	}

	/// Check if a timestamp is in the past
	pub fn is_past(timestamp: &DateTime<Utc>) -> bool {
		*timestamp < Utc::now()
	}

	/// Check if a timestamp is in the future
	pub fn is_future(timestamp: &DateTime<Utc>) -> bool {
		*timestamp > Utc::now()
	}

	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_time_utilities() {
			let now = now_utc();
			let future = hours_from_now(1);
			let past = now - Duration::hours(1);

			assert!(is_past(&past));
			assert!(is_future(&future));
			assert!(!is_past(&future));
			assert!(!is_future(&past));

			let formatted = format_timestamp(&now);
			assert!(formatted.contains("UTC"));
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_constants() {
		assert_eq!(constants::DEFAULT_PORT, 4000);
		assert_eq!(constants::APP_NAME, "La Liga de los Muertos");
		assert_eq!(constants::API_VERSION, "v1");
	}
}
