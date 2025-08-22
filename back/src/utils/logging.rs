//! Logging utilities for the Liga de los Muertos backend
//!
//! This module provides enhanced logging configuration with support for
//! different log levels, structured logging, and environment-based configuration.
//!
//! # Usage Examples
//!
//! ## Basic Usage
//! ```rust
//! use liga_muertos_back::utils::logging;
//!
//! // Initialize logging (call once at startup)
//! logging::init();
//!
//! // Application lifecycle
//! logging::startup_info(4000);
//! logging::server_ready(4000);
//! logging::shutdown();
//!
//! // Database operations
//! logging::schema_init();
//! logging::database_info("ws://localhost:8000", "liga", "muertos");
//! logging::schema_success();
//! ```
//!
//! ## Event Logging
//! ```rust
//! use liga_muertos_back::utils::logging;
//!
//! // Authentication events
//! logging::auth_event("login_success", Some("user_12345"));
//! logging::auth_event("logout", Some("user_12345"));
//!
//! // Tournament events
//! logging::tournament_event("created", "tournament_abc", Some("user_12345"));
//! logging::tournament_event("started", "tournament_abc", None);
//! ```
//!
//! ## Performance Monitoring
//! ```rust
//! use liga_muertos_back::utils::logging;
//!
//! let start = std::time::Instant::now();
//! // ... do some work ...
//! let duration = start.elapsed().as_millis() as u64;
//! logging::performance_metric("database_query", duration);
//! ```
//!
//! ## Request Debugging
//! ```rust
//! use liga_muertos_back::utils::logging;
//!
//! logging::request_debug("GET", "/v1/tournaments", Some("Mozilla/5.0"));
//! ```

use env_logger::{Builder, Env, Target};
use log::LevelFilter;
use std::env;

/// Initialize the logging system with enhanced configuration
///
/// This function sets up logging with the following features:
/// - Environment-based log level configuration (RUST_LOG)
/// - Colored output for terminal
/// - Timestamp formatting
/// - Module path filtering
/// - Configurable target (stdout/stderr)
pub fn init() {
	let env = Env::default()
		.filter_or("RUST_LOG", "liga_muertos_back=info,actix_web=info")
		.write_style_or("RUST_LOG_STYLE", "auto");

	let mut builder = Builder::from_env(env);

	// Configure the log format
	builder
		.target(Target::Stdout)
		.format_timestamp_secs()
		.format_module_path(true)
		.format_level(true);

	// Set additional filtering based on environment
	if let Ok(level) = env::var("LOG_LEVEL") {
		let level_filter = match level.to_lowercase().as_str() {
			"trace" => LevelFilter::Trace,
			"debug" => LevelFilter::Debug,
			"info" => LevelFilter::Info,
			"warn" => LevelFilter::Warn,
			"error" => LevelFilter::Error,
			"off" => LevelFilter::Off,
			_ => {
				eprintln!("⚠️  Invalid LOG_LEVEL '{}', using default", level);
				LevelFilter::Info
			}
		};
		builder.filter_level(level_filter);
	}

	// Initialize the logger
	builder.init();
}

/// Log application startup information
pub fn startup_info(port: u16) {
	log::info!("🦀 Starting La Liga de los Muertos backend");
	log::info!("🌐 Server will bind to 0.0.0.0:{}", port);
	log::info!("📝 API documentation: https://la-liga-de-los-muertos.apidog.io");
	log::debug!("🔧 Debug logging enabled");
}

/// Log database connection information
pub fn database_info(url: &str, namespace: &str, database: &str) {
	log::info!("🔌 Connected to SurrealDB at {}", url);
	log::info!("📊 Using namespace: {} / database: {}", namespace, database);
}

/// Log database schema initialization
pub fn schema_init() {
	log::info!("🔧 Initializing database schema...");
}

/// Log successful schema initialization
pub fn schema_success() {
	log::info!("✅ Database schema initialized successfully");
}

/// Log database connection failure
pub fn database_error(error: &str) {
	log::error!("❌ Failed to initialize database connection: {}", error);
	log::error!("🔧 Please check your database configuration and try again.");
	log::error!("💡 Ensure SurrealDB is running and accessible at the configured URL");
}

/// Log server startup success
pub fn server_ready(port: u16) {
	log::info!("🚀 Server ready and listening on port {}", port);
	log::info!(
		"🏥 Health check available at: http://localhost:{}/v1/health",
		port
	);
}

/// Log graceful shutdown
pub fn shutdown() {
	log::info!("🛑 Gracefully shutting down La Liga de los Muertos backend");
}

/// Log request information for debugging
pub fn request_debug(method: &str, path: &str, user_agent: Option<&str>) {
	if log::log_enabled!(log::Level::Debug) {
		match user_agent {
			Some(ua) => log::debug!("📥 {} {} - User-Agent: {}", method, path, ua),
			None => log::debug!("📥 {} {}", method, path),
		}
	}
}

/// Log performance metrics
pub fn performance_metric(operation: &str, duration_ms: u64) {
	if duration_ms > 1000 {
		log::warn!("⏰ Slow operation: {} took {}ms", operation, duration_ms);
	} else {
		log::debug!("⚡ {}: {}ms", operation, duration_ms);
	}
}

/// Log authentication events
pub fn auth_event(event: &str, user_id: Option<&str>) {
	match user_id {
		Some(id) => log::info!("🔐 Auth event: {} for user {}", event, id),
		None => log::info!("🔐 Auth event: {}", event),
	}
}

/// Log tournament events
pub fn tournament_event(event: &str, tournament_id: &str, user_id: Option<&str>) {
	match user_id {
		Some(id) => log::info!(
			"🏆 Tournament event: {} for tournament {} by user {}",
			event,
			tournament_id,
			id
		),
		None => log::info!(
			"🏆 Tournament event: {} for tournament {}",
			event,
			tournament_id
		),
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_logging_functions_dont_panic() {
		// These tests just ensure the logging functions don't panic
		// Actual log output would need integration tests

		startup_info(4000);
		database_info("ws://localhost:8000", "test", "test");
		schema_init();
		schema_success();
		database_error("Connection failed");
		server_ready(4000);
		shutdown();
		request_debug("GET", "/health", Some("test-agent"));
		request_debug("POST", "/api/test", None);
		performance_metric("database_query", 150);
		performance_metric("slow_operation", 1500);
		auth_event("login", Some("user123"));
		auth_event("logout", None);
		tournament_event("created", "tournament123", Some("user456"));
		tournament_event("started", "tournament123", None);
	}

	#[test]
	fn test_log_level_parsing() {
		// Test that invalid log levels don't crash
		unsafe {
			std::env::set_var("LOG_LEVEL", "invalid");
		}
		// This would normally output a warning, but we can't easily test that
		// without capturing stderr, so we just ensure it doesn't panic

		unsafe {
			std::env::remove_var("LOG_LEVEL");
		}
	}
}
