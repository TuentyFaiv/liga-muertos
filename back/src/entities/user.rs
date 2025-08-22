//! User entity definitions for authentication and user management

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

/// Full user record as stored in the database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
	pub id: RecordId,
	pub username: String,
	pub email: String,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
}

/// Data for creating a new user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserData {
	pub username: String,
	pub email: String,
}

/// Data for updating an existing user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserData {
	pub username: Option<String>,
	pub email: Option<String>,
}

/// Public user information (without sensitive data)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicUser {
	pub id: RecordId,
	pub username: String,
	pub created_at: DateTime<Utc>,
}

impl From<User> for PublicUser {
	fn from(user: User) -> Self {
		Self {
			id: user.id,
			username: user.username,
			created_at: user.created_at,
		}
	}
}

/// User authentication credentials
#[derive(Debug, Clone, Deserialize)]
pub struct UserCredentials {
	pub username: String,
	pub password: String,
}

/// User registration data
#[derive(Debug, Clone, Deserialize)]
pub struct UserRegistration {
	pub username: String,
	pub email: String,
	pub password: String,
}

#[cfg(test)]
mod tests {
	use super::*;
	use chrono::Utc;
	use surrealdb::RecordId;

	#[test]
	fn test_user_to_public_user_conversion() {
		let user = User {
			id: RecordId::from(("user", "test123")),
			username: "testuser".to_string(),
			email: "test@example.com".to_string(),
			created_at: Utc::now(),
			updated_at: Utc::now(),
		};

		let public_user: PublicUser = user.clone().into();

		assert_eq!(public_user.id, user.id);
		assert_eq!(public_user.username, user.username);
		assert_eq!(public_user.created_at, user.created_at);
		// Email should not be in public user
	}

	#[test]
	fn test_create_user_data_serialization() {
		let data = CreateUserData {
			username: "newuser".to_string(),
			email: "new@example.com".to_string(),
		};

		let json = serde_json::to_string(&data).unwrap();
		let deserialized: CreateUserData = serde_json::from_str(&json).unwrap();

		assert_eq!(data.username, deserialized.username);
		assert_eq!(data.email, deserialized.email);
	}

	#[test]
	fn test_update_user_data_partial() {
		let data = UpdateUserData {
			username: Some("updateduser".to_string()),
			email: None,
		};

		assert!(data.username.is_some());
		assert!(data.email.is_none());
	}
}
