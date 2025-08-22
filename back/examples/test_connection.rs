//! SurrealDB Connection Test Utility
//!
//! This utility helps debug connection issues with SurrealDB Cloud
//! by testing different connection formats and providing detailed error information.
//!
//! Run with: `cargo run --example test_connection`

use dotenvy::dotenv;
use std::env;
use std::sync::LazyLock;
use surrealdb::{
	Surreal,
	engine::remote::ws::{Client, Ws, Wss},
	opt::auth::Root,
};

static DB: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	// Load environment variables
	dotenv().ok();

	println!("🔍 SurrealDB Connection Tester");
	println!("================================");

	// Get configuration from environment
	let db_url = env::var("SURREAL_URL").unwrap_or_default();
	let namespace = env::var("SURREAL_NAMESPACE").unwrap_or("test".to_string());
	let database = env::var("SURREAL_DATABASE").unwrap_or("test".to_string());
	let username = env::var("SURREAL_USER").unwrap_or("root".to_string());
	let password = env::var("SURREAL_PASS").unwrap_or("root".to_string());

	println!("📋 Configuration:");
	println!("  URL: {}", db_url);
	println!("  Namespace: {}", namespace);
	println!("  Database: {}", database);
	println!("  Username: {}", username);
	println!("  Password: [{}]", "*".repeat(password.len().min(8)));
	println!();

	if db_url.is_empty() {
		println!("❌ SURREAL_URL is not set in .env file");
		return Ok(());
	}

	// Test different URL formats
	let test_urls = generate_test_urls(&db_url);

	for (i, test_url) in test_urls.iter().enumerate() {
		println!("🧪 Test {} - Trying URL: {}", i + 1, test_url);

		let result = test_connection(test_url, &namespace, &database, &username, &password).await;

		match result {
			Ok(_) => {
				println!("✅ SUCCESS! Connection established with: {}", test_url);
				println!("🎉 Use this URL in your .env file:");
				println!("SURREAL_URL={}", test_url);
				println!();
				println!("🔧 Also check that your credentials are correct:");
				println!("SURREAL_NAMESPACE={}", namespace);
				println!("SURREAL_DATABASE={}", database);
				println!("SURREAL_USER={}", username);
				return Ok(());
			}
			Err(e) => {
				println!("❌ Failed: {}", e);
				println!();
			}
		}
	}

	println!("🚨 All connection attempts failed!");
	println!();
	println!("🔧 Troubleshooting suggestions:");
	println!("1. Check that your SurrealDB Cloud instance is running and active");
	println!("2. Verify your credentials are correct in SurrealDB Cloud dashboard");
	println!("3. Ensure your IP is whitelisted (SurrealDB Cloud requires IP whitelisting)");
	println!("4. Check the exact URL format from SurrealDB Cloud dashboard");
	println!("5. Try using the HTTP endpoint instead of WebSocket if available");
	println!("6. Verify your network connection and firewall settings");
	println!("7. Check SurrealDB Cloud status page for any outages");
	println!();
	println!("💡 Common SurrealDB Cloud URL formats:");
	println!("   - wss://your-instance.surreal.cloud/rpc");
	println!("   - wss://your-instance.surreal.cloud:8000/rpc");
	println!("   - https://your-instance.surreal.cloud/sql");

	Ok(())
}

fn generate_test_urls(original_url: &str) -> Vec<String> {
	let mut urls = Vec::new();

	// Remove any existing /rpc suffix
	let base_url = original_url.trim_end_matches("/rpc");

	// Test different combinations for SurrealDB Cloud
	urls.push(format!("{}/rpc", base_url)); // Most common format
	urls.push(format!("{}:8000/rpc", base_url)); // With port
	urls.push(base_url.to_string()); // Original without /rpc
	urls.push(format!("{}:8000", base_url)); // With port, no /rpc

	// If it's wss://, also try ws:// variants
	if base_url.starts_with("wss://") {
		let ws_base = base_url.replace("wss://", "ws://");
		urls.push(format!("{}/rpc", ws_base));
		urls.push(format!("{}:8000/rpc", ws_base));
		urls.push(ws_base.clone());
		urls.push(format!("{}:8000", ws_base));
	}

	// If it's ws://, also try wss:// variants
	if base_url.starts_with("ws://") {
		let wss_base = base_url.replace("ws://", "wss://");
		urls.push(format!("{}/rpc", wss_base));
		urls.push(format!("{}:8000/rpc", wss_base));
		urls.push(wss_base.clone());
		urls.push(format!("{}:8000", wss_base));
	}

	// Add HTTP endpoints for SurrealDB Cloud
	if base_url.starts_with("wss://") {
		let https_base = base_url.replace("wss://", "https://");
		urls.push(format!("{}/sql", https_base));
		urls.push(format!("{}:8000/sql", https_base));
	} else if base_url.starts_with("ws://") {
		let http_base = base_url.replace("ws://", "http://");
		urls.push(format!("{}/sql", http_base));
		urls.push(format!("{}:8000/sql", http_base));
	}

	urls
}

async fn test_connection(
	url: &str,
	namespace: &str,
	database: &str,
	username: &str,
	password: &str,
) -> Result<(), Box<dyn std::error::Error>> {
	// Try connection based on protocol
	if url.starts_with("wss://") || url.starts_with("ws://") {
		// Test WebSocket connection
		if url.starts_with("wss://") {
			DB.connect::<Wss>(url).await?;
		} else {
			DB.connect::<Ws>(url).await?;
		}

		// Test authentication
		DB.signin(Root { username, password }).await?;

		// Test namespace/database selection
		DB.use_ns(namespace).use_db(database).await?;

		// Test a simple query
		let mut result = DB.query("RETURN 'connection_test'").await?;
		let _greeting: Option<String> = result.take(0)?;
	} else if url.starts_with("https://") || url.starts_with("http://") {
		// For HTTP endpoints, we'll just test if we can reach them
		// This is a basic connectivity test
		return Err("HTTP endpoints require different connection method".into());
	} else {
		return Err("URL must start with ws://, wss://, http://, or https://".into());
	}

	Ok(())
}
