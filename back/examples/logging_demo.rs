//! Logging Demo Example
//!
//! This example demonstrates the clean and intuitive logging API provided
//! by the Liga de los Muertos backend utilities.
//!
//! Run this example with:
//! ```bash
//! RUST_LOG=debug cargo run --example logging_demo
//! ```

use liga_muertos_back::utils::logging;
use std::time::Instant;

fn main() {
	// Initialize the logging system
	logging::init();

	// Demonstrate startup logging
	logging::startup_info(4000);

	// Simulate database operations
	simulate_database_operations();

	// Simulate authentication events
	simulate_auth_events();

	// Simulate tournament operations
	simulate_tournament_operations();

	// Simulate performance monitoring
	simulate_performance_monitoring();

	// Demonstrate server ready
	logging::server_ready(4000);

	// Demonstrate graceful shutdown
	logging::shutdown();
}

fn simulate_database_operations() {
	println!("\n=== Database Operations Demo ===");

	// Simulate schema initialization
	logging::schema_init();

	// Simulate a delay (like real DB operations)
	std::thread::sleep(std::time::Duration::from_millis(100));

	logging::database_info("ws://localhost:8000", "liga", "muertos");
	logging::schema_success();

	// Simulate an error scenario (commented out to not terminate the demo)
	// logging::database_error("Connection timeout");
}

fn simulate_auth_events() {
	println!("\n=== Authentication Events Demo ===");

	logging::auth_event("user_registration", Some("user_12345"));
	logging::auth_event("login_attempt", Some("user_12345"));
	logging::auth_event("login_success", Some("user_12345"));
	logging::auth_event("token_refresh", Some("user_12345"));
	logging::auth_event("logout", Some("user_12345"));
	logging::auth_event("failed_login_attempt", None);
}

fn simulate_tournament_operations() {
	println!("\n=== Tournament Events Demo ===");

	logging::tournament_event("created", "tournament_abc123", Some("user_12345"));
	logging::tournament_event("published", "tournament_abc123", Some("user_12345"));
	logging::tournament_event(
		"participant_joined",
		"tournament_abc123",
		Some("user_67890"),
	);
	logging::tournament_event(
		"participant_joined",
		"tournament_abc123",
		Some("user_11111"),
	);
	logging::tournament_event("started", "tournament_abc123", None);
	logging::tournament_event("match_completed", "tournament_abc123", None);
	logging::tournament_event("completed", "tournament_abc123", None);
}

fn simulate_performance_monitoring() {
	println!("\n=== Performance Monitoring Demo ===");

	// Simulate fast operations
	let start = Instant::now();
	std::thread::sleep(std::time::Duration::from_millis(50));
	let duration = start.elapsed().as_millis() as u64;
	logging::performance_metric("database_query", duration);

	let start = Instant::now();
	std::thread::sleep(std::time::Duration::from_millis(200));
	let duration = start.elapsed().as_millis() as u64;
	logging::performance_metric("api_request", duration);

	// Simulate slow operation (will trigger warning)
	let start = Instant::now();
	std::thread::sleep(std::time::Duration::from_millis(1200));
	let duration = start.elapsed().as_millis() as u64;
	logging::performance_metric("slow_tournament_calculation", duration);

	// Simulate request debugging
	logging::request_debug("GET", "/v1/tournaments", Some("Mozilla/5.0"));
	logging::request_debug("POST", "/v1/tournaments", Some("curl/7.68.0"));
	logging::request_debug("PUT", "/v1/tournaments/123", None);
	logging::request_debug(
		"DELETE",
		"/v1/tournaments/456",
		Some("PostmanRuntime/7.29.0"),
	);
}

// Example of how you might use logging in actual application code
pub struct TournamentService;

impl TournamentService {
	pub fn create_tournament(&self, _name: &str, creator_id: &str) -> Result<String, String> {
		let start = Instant::now();

		// Simulate tournament creation logic
		std::thread::sleep(std::time::Duration::from_millis(150));

		let tournament_id = format!("tournament_{}", uuid::Uuid::new_v4().simple());

		// Log the successful creation
		logging::tournament_event("created", &tournament_id, Some(creator_id));

		// Log performance
		let duration = start.elapsed().as_millis() as u64;
		logging::performance_metric("tournament_creation", duration);

		Ok(tournament_id)
	}

	pub fn join_tournament(&self, tournament_id: &str, user_id: &str) -> Result<(), String> {
		let start = Instant::now();

		// Simulate join logic
		std::thread::sleep(std::time::Duration::from_millis(75));

		// Log the event
		logging::tournament_event("participant_joined", tournament_id, Some(user_id));

		// Log performance
		let duration = start.elapsed().as_millis() as u64;
		logging::performance_metric("tournament_join", duration);

		Ok(())
	}
}

// Example of how you might use logging in authentication code
pub struct AuthService;

impl AuthService {
	pub fn authenticate_user(&self, _username: &str) -> Result<String, String> {
		let start = Instant::now();

		// Simulate authentication logic
		std::thread::sleep(std::time::Duration::from_millis(100));

		let user_id = format!("user_{}", uuid::Uuid::new_v4().simple());

		// Log successful authentication
		logging::auth_event("login_success", Some(&user_id));

		// Log performance
		let duration = start.elapsed().as_millis() as u64;
		logging::performance_metric("user_authentication", duration);

		Ok(user_id)
	}

	pub fn logout_user(&self, user_id: &str) {
		logging::auth_event("logout", Some(user_id));
	}
}

// Add uuid dependency for realistic IDs
mod uuid {
	pub struct Uuid;
	impl Uuid {
		pub fn new_v4() -> Self {
			Self
		}
		pub fn simple(&self) -> String {
			format!("{:08x}", rand::random::<u32>())
		}
	}

	mod rand {
		pub fn random<T>() -> T
		where
			T: Default,
		{
			T::default()
		}
	}
}
