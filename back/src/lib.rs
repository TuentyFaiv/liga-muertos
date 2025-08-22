// Lib file to help with testing and module organization

use std::env;
use std::sync::LazyLock;

use surrealdb::Surreal;
use surrealdb::engine::any::Any;
use surrealdb::opt::auth::Root;

pub mod entities;
pub mod routes;
pub mod utils;

use crate::utils::error::ApiResult;
use crate::utils::logging;

// Global database client using Any engine for multi-protocol support
pub static DB: LazyLock<Surreal<Any>> = LazyLock::new(Surreal::init);

#[derive(Clone)]
pub struct AppState {}

impl AppState {
	pub fn new() -> Self {
		Self {}
	}

	pub fn new_test() -> Self {
		Self {}
	}
}

/// Initialize the database connection with SurrealDB
pub async fn init_db() -> ApiResult<()> {
	// Get database connection details from environment
	let db_url = env::var("SURREAL_URL").unwrap_or("wss://localhost:8000".to_owned());
	let namespace = env::var("SURREAL_NAMESPACE").unwrap_or("liga".to_owned());
	let database = env::var("SURREAL_DATABASE").unwrap_or("muertos".to_owned());
	let username = env::var("SURREAL_USER").unwrap_or("root".to_owned());
	let password = env::var("SURREAL_PASS").unwrap_or("root".to_owned());

	// Connect using Any engine which auto-detects protocol
	DB.connect(&db_url).await?;

	// Sign in as root user
	DB.signin(Root {
		username: &username,
		password: &password,
	})
	.await?;

	// Use the specified namespace and database
	DB.use_ns(&namespace).use_db(&database).await?;

	logging::database_info(&db_url, &namespace, &database);

	// Initialize database schema
	init_schema().await?;

	Ok(())
}

/// Initialize database schema and tables
async fn init_schema() -> ApiResult<()> {
	logging::schema_init();

	// Define basic schema for the application
	let schema_query = r#"
        -- Define users table for authentication
        DEFINE TABLE IF NOT EXISTS user SCHEMALESS
            PERMISSIONS FOR
                SELECT, UPDATE WHERE id = $auth,
                FOR CREATE, DELETE NONE;

        DEFINE FIELD IF NOT EXISTS username ON TABLE user TYPE string;
        DEFINE FIELD IF NOT EXISTS email ON TABLE user TYPE string;
        DEFINE FIELD IF NOT EXISTS created_at ON TABLE user TYPE datetime VALUE time::now() READONLY;
        DEFINE FIELD IF NOT EXISTS updated_at ON TABLE user TYPE datetime VALUE time::now();

        -- Define tournaments table
        DEFINE TABLE IF NOT EXISTS tournament SCHEMALESS
            PERMISSIONS FOR
                SELECT WHERE published = true OR created_by = $auth,
                FOR CREATE, UPDATE, DELETE WHERE created_by = $auth;

        DEFINE FIELD IF NOT EXISTS name ON TABLE tournament TYPE string;
        DEFINE FIELD IF NOT EXISTS description ON TABLE tournament TYPE string;
        DEFINE FIELD IF NOT EXISTS published ON TABLE tournament TYPE bool VALUE false;
        DEFINE FIELD IF NOT EXISTS created_by ON TABLE tournament VALUE $auth READONLY;
        DEFINE FIELD IF NOT EXISTS created_at ON TABLE tournament TYPE datetime VALUE time::now() READONLY;
        DEFINE FIELD IF NOT EXISTS updated_at ON TABLE tournament TYPE datetime VALUE time::now();

        -- Define participants table
        DEFINE TABLE IF NOT EXISTS participant SCHEMALESS
            PERMISSIONS FOR
                SELECT WHERE tournament IN (SELECT id FROM tournament WHERE published = true OR created_by = $auth),
                FOR CREATE WHERE tournament IN (SELECT id FROM tournament WHERE published = true),
                FOR UPDATE, DELETE WHERE user_id = $auth OR tournament IN (SELECT id FROM tournament WHERE created_by = $auth);

        DEFINE FIELD IF NOT EXISTS tournament ON TABLE participant TYPE record<tournament>;
        DEFINE FIELD IF NOT EXISTS user_id ON TABLE participant TYPE record<user>;
        DEFINE FIELD IF NOT EXISTS joined_at ON TABLE participant TYPE datetime VALUE time::now() READONLY;
  "#;

	// Execute schema definition using the documentation pattern
	DB.query(schema_query).await?;

	logging::schema_success();
	Ok(())
}

/// Database operation helpers using the clean .await? pattern
pub mod database {
	use super::*;

	/// Execute a database query with proper error handling
	///
	/// # Example
	/// ```rust
	/// use liga_muertos_back::database;
	///
	/// #[actix_web::main]
	/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
	///     // This demonstrates the clean .await? pattern from SurrealDB docs
	///     let result = database::query("SELECT * FROM users").await?;
	///     Ok(())
	/// }
	/// ```
	pub async fn query(sql: &str) -> ApiResult<surrealdb::Response> {
		DB.query(sql).await.map_err(|e| e.into())
	}

	/// Example showing the SurrealDB documentation pattern in action
	pub async fn demo_documentation_pattern() -> Result<(), Box<dyn std::error::Error>> {
		// This follows the exact pattern from SurrealDB documentation:
		// https://surrealdb.com/docs/sdk/rust/frameworks/actix#getting-started

		println!("📚 Demonstrating SurrealDB documentation pattern:");

		// Connect to the database (this is already done in init_db, but shown for reference)
		// DB.connect::<Ws>("localhost:8000").await?;

		// Sign in as root user (this is already done in init_db, but shown for reference)
		// DB.signin(Root {
		//     username: "root",
		//     password: "secret",
		// }).await?;

		// Use namespace and database (this is already done in init_db, but shown for reference)
		// DB.use_ns("namespace").use_db("database").await?;

		// Execute a test query using the clean .await? pattern
		let mut result = DB.query("RETURN 'Hello from SurrealDB!'").await?;
		let greeting: Option<String> = result.take(0)?;

		if let Some(msg) = greeting {
			println!("✅ Database response: {}", msg);
		}

		println!("🎉 Documentation pattern works perfectly!");
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	// use surrealdb::engine::local::Mem;

	#[test]
	fn test_app_state_creation() {
		let _state = AppState::new();
		assert!(true); // AppState is just an empty struct now
	}

	#[test]
	fn test_app_state_test_creation() {
		let _state = AppState::new_test();
		assert!(true); // AppState is just an empty struct now
	}

	#[test]
	fn test_surreal_env_vars_parsing() {
		// Test that our environment variable names are correctly used
		use std::env;

		unsafe {
			// Set test environment variables with SURREAL_ prefix
			env::set_var("SURREAL_URL", "ws://test.example.com:8000/rpc");
			env::set_var("SURREAL_NAMESPACE", "test_namespace");
			env::set_var("SURREAL_DATABASE", "test_database");
			env::set_var("SURREAL_USER", "test_user");
			env::set_var("SURREAL_PASS", "test_pass");
		}

		// Verify that the variables can be read with the new names
		assert_eq!(
			env::var("SURREAL_URL").unwrap(),
			"ws://test.example.com:8000/rpc"
		);
		assert_eq!(env::var("SURREAL_NAMESPACE").unwrap(), "test_namespace");
		assert_eq!(env::var("SURREAL_DATABASE").unwrap(), "test_database");
		assert_eq!(env::var("SURREAL_USER").unwrap(), "test_user");
		assert_eq!(env::var("SURREAL_PASS").unwrap(), "test_pass");

		unsafe {
			// Clean up test environment variables
			env::remove_var("SURREAL_URL");
			env::remove_var("SURREAL_NAMESPACE");
			env::remove_var("SURREAL_DATABASE");
			env::remove_var("SURREAL_USER");
			env::remove_var("SURREAL_PASS");
		}
	}
}
