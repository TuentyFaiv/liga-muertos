//! Validation error module for input validation
//!
//! This module provides specific validation errors and utilities for
//! validating user input, request data, and business logic constraints.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

/// Validation error for field-specific validation failures
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
#[error("Validation error: {message}")]
pub struct ValidationError {
	/// Human-readable error message
	pub message: String,
	/// Field that failed validation (optional)
	pub field: Option<String>,
	/// Error code for programmatic handling
	pub code: String,
}

impl ValidationError {
	/// Create a new validation error
	pub fn new(message: &str, code: &str) -> Self {
		Self {
			message: message.to_string(),
			field: None,
			code: code.to_string(),
		}
	}

	/// Create a validation error with field
	pub fn with_field(message: &str, field: &str, code: &str) -> Self {
		Self {
			message: message.to_string(),
			field: Some(field.to_string()),
			code: code.to_string(),
		}
	}
}

/// Multiple validation errors for comprehensive input validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationErrors {
	/// List of validation errors
	pub errors: Vec<ValidationError>,
}

impl ValidationErrors {
	/// Create a new validation errors collection
	pub fn new() -> Self {
		Self { errors: Vec::new() }
	}

	/// Add a validation error
	pub fn add(&mut self, error: ValidationError) {
		self.errors.push(error);
	}

	/// Add a simple validation error
	pub fn add_error(&mut self, message: &str, field: &str, code: &str) {
		self.add(ValidationError::with_field(message, field, code));
	}

	/// Check if there are any errors
	pub fn has_errors(&self) -> bool {
		!self.errors.is_empty()
	}

	/// Get the first error (if any)
	pub fn first(&self) -> Option<&ValidationError> {
		self.errors.first()
	}

	/// Convert to a map of field -> error messages
	pub fn to_field_map(&self) -> HashMap<String, Vec<String>> {
		let mut map = HashMap::new();

		for error in &self.errors {
			let field = error.field.as_deref().unwrap_or("_general");
			map
				.entry(field.to_string())
				.or_insert_with(Vec::new)
				.push(error.message.clone());
		}

		map
	}
}

impl Default for ValidationErrors {
	fn default() -> Self {
		Self::new()
	}
}

/// Validation result type
pub type ValidationResult<T> = Result<T, ValidationErrors>;

/// Common validation functions
pub mod validators {
	use super::ValidationError;
	use crate::utils::validation::*;

	/// Validate required field
	pub fn required<'a>(value: Option<&'a str>, field: &str) -> Result<&'a str, ValidationError> {
		match value {
			Some(v) if !v.trim().is_empty() => Ok(v.trim()),
			_ => Err(ValidationError::with_field(
				&format!("{field} is required"),
				field,
				"REQUIRED",
			)),
		}
	}

	/// Validate string length
	pub fn length(value: &str, min: usize, max: usize, field: &str) -> Result<(), ValidationError> {
		let len = value.len();
		if len < min || len > max {
			Err(ValidationError::with_field(
				&format!("{field} must be between {min} and {max} characters"),
				field,
				"LENGTH",
			))
		} else {
			Ok(())
		}
	}

	/// Validate email format
	pub fn email(value: &str, field: &str) -> Result<(), ValidationError> {
		if is_valid_email(value) {
			Ok(())
		} else {
			Err(ValidationError::with_field(
				"Invalid email format",
				field,
				"INVALID_EMAIL",
			))
		}
	}

	/// Validate username format
	pub fn username(value: &str, field: &str) -> Result<(), ValidationError> {
		if is_valid_username(value) {
			Ok(())
		} else {
			Err(ValidationError::with_field(
				"Username must be 3-50 characters, start with alphanumeric, and contain only alphanumeric, hyphens, or underscores",
				field,
				"INVALID_USERNAME",
			))
		}
	}

	/// Validate password strength
	pub fn password(value: &str, field: &str) -> Result<(), ValidationError> {
		if is_valid_password(value) {
			Ok(())
		} else {
			Err(ValidationError::with_field(
				"Password must be 8-128 characters with at least one lowercase, uppercase, and digit",
				field,
				"WEAK_PASSWORD",
			))
		}
	}

	/// Validate tournament name
	pub fn tournament_name(value: &str, field: &str) -> Result<(), ValidationError> {
		if is_valid_tournament_name(value) {
			Ok(())
		} else {
			Err(ValidationError::with_field(
				"Tournament name must be 3-100 characters and not empty",
				field,
				"INVALID_TOURNAMENT_NAME",
			))
		}
	}

	/// Validate UUID format
	pub fn uuid_format(value: &str, field: &str) -> Result<(), ValidationError> {
		// Simple UUID format validation
		if value.len() == 36 && value.chars().filter(|&c| c == '-').count() == 4 {
			Ok(())
		} else {
			Err(ValidationError::with_field(
				"Invalid UUID format",
				field,
				"INVALID_UUID",
			))
		}
	}

	/// Validate positive integer
	pub fn positive_integer(value: i32, field: &str) -> Result<(), ValidationError> {
		if value > 0 {
			Ok(())
		} else {
			Err(ValidationError::with_field(
				&format!("{field} must be a positive integer"),
				field,
				"NOT_POSITIVE",
			))
		}
	}

	/// Validate range
	pub fn range(value: i32, min: i32, max: i32, field: &str) -> Result<(), ValidationError> {
		if value >= min && value <= max {
			Ok(())
		} else {
			Err(ValidationError::with_field(
				&format!("{field} must be between {min} and {max}"),
				field,
				"OUT_OF_RANGE",
			))
		}
	}
}

/// Validation builder for complex validation scenarios
pub struct ValidationBuilder {
	errors: ValidationErrors,
}

impl ValidationBuilder {
	/// Create a new validation builder
	pub fn new() -> Self {
		Self {
			errors: ValidationErrors::new(),
		}
	}

	/// Validate a field and add any errors to the collection
	pub fn validate<F>(mut self, validation: F) -> Self
	where
		F: FnOnce() -> Result<(), ValidationError>,
	{
		if let Err(error) = validation() {
			self.errors.add(error);
		}
		self
	}

	/// Build the final result
	pub fn build<T>(self, success_value: T) -> ValidationResult<T> {
		if self.errors.has_errors() {
			Err(self.errors)
		} else {
			Ok(success_value)
		}
	}

	/// Build the final result without a success value
	pub fn build_unit(self) -> ValidationResult<()> {
		self.build(())
	}
}

impl Default for ValidationBuilder {
	fn default() -> Self {
		Self::new()
	}
}

/// Macro for convenient validation
#[macro_export]
macro_rules! validate {
    ($($validation:expr),* $(,)?) => {
        {
            let mut builder = $crate::utils::error::validation::ValidationBuilder::new();
            $(
                builder = builder.validate(|| $validation);
            )*
            builder.build_unit()
        }
    };
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::utils::error::validation::validators::*;

	#[test]
	fn test_validation_error_creation() {
		let error = ValidationError::new("Test error", "TEST");
		assert_eq!(error.message, "Test error");
		assert_eq!(error.code, "TEST");
		assert!(error.field.is_none());

		let error_with_field = ValidationError::with_field("Field error", "username", "INVALID");
		assert_eq!(error_with_field.message, "Field error");
		assert_eq!(error_with_field.code, "INVALID");
		assert_eq!(error_with_field.field.as_deref(), Some("username"));
	}

	#[test]
	fn test_validation_errors_collection() {
		let mut errors = ValidationErrors::new();
		assert!(!errors.has_errors());

		errors.add_error("Username required", "username", "REQUIRED");
		errors.add_error("Email invalid", "email", "INVALID_EMAIL");

		assert!(errors.has_errors());
		assert_eq!(errors.errors.len(), 2);

		let field_map = errors.to_field_map();
		assert!(field_map.contains_key("username"));
		assert!(field_map.contains_key("email"));
	}

	#[test]
	fn test_validators_required() {
		assert!(required(Some("test"), "field").is_ok());
		assert!(required(Some(""), "field").is_err());
		assert!(required(None, "field").is_err());
	}

	#[test]
	fn test_validators_length() {
		assert!(length("hello", 3, 10, "field").is_ok());
		assert!(length("hi", 3, 10, "field").is_err());
		assert!(length("this is too long for the limit", 3, 10, "field").is_err());
	}

	#[test]
	fn test_validators_positive_integer() {
		assert!(positive_integer(5, "field").is_ok());
		assert!(positive_integer(0, "field").is_err());
		assert!(positive_integer(-1, "field").is_err());
	}

	#[test]
	fn test_validators_range() {
		assert!(range(5, 1, 10, "field").is_ok());
		assert!(range(0, 1, 10, "field").is_err());
		assert!(range(11, 1, 10, "field").is_err());
	}

	#[test]
	fn test_validation_builder() {
		let result = ValidationBuilder::new()
			.validate(|| required(Some("test"), "username").map(|_| ()))
			.validate(|| length("test", 3, 10, "username"))
			.build("success");

		assert!(result.is_ok());
		assert_eq!(result.unwrap(), "success");

		let result = ValidationBuilder::new()
			.validate(|| required(None, "username").map(|_| ()))
			.validate(|| length("", 3, 10, "username"))
			.build_unit();

		assert!(result.is_err());
		let errors = result.unwrap_err();
		assert!(errors.has_errors());
		assert!(!errors.errors.is_empty());
	}

	#[test]
	fn test_validation_macro() {
		let username = "test";
		let result = validate!(
			required(Some(username), "username").map(|_| ()),
			length(username, 3, 10, "username")
		);
		assert!(result.is_ok());

		let result = validate!(
			required(None, "username").map(|_| ()),
			positive_integer(-1, "age")
		);
		assert!(result.is_err());
	}
}
