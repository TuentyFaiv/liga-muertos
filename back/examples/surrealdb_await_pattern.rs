//! SurrealDB .await? Pattern Demo
//!
//! This example demonstrates how to use SurrealDB with the clean `.await?` pattern
//! as shown in the official documentation, integrated with our error handling system.
//!
//! Run with: `cargo run --example surrealdb_await_pattern`

use liga_muertos_back::{DB, database, utils::logging};
use serde::{Deserialize, Serialize};
use std::time::Instant;

#[derive(Debug, Serialize, Deserialize)]
struct Person {
	id: Option<surrealdb::RecordId>,
	name: String,
	age: u8,
}

#[derive(Debug, Serialize, Deserialize)]
struct PersonData {
	name: String,
	age: u8,
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	// Initialize logging
	logging::init();

	println!("🦀 SurrealDB .await? Pattern Demo");
	println!("==================================");

	// Example 1: Basic SurrealDB documentation pattern
	demo_basic_pattern().await?;

	// Example 2: CRUD operations with clean error handling
	demo_crud_operations().await?;

	// Example 3: Complex queries with proper error propagation
	demo_complex_queries().await?;

	// Example 4: Direct DB operations with clean patterns
	demo_direct_operations().await?;

	// Example 5: Transaction-like operations
	demo_transactions().await?;

	println!("✅ All demos completed successfully!");
	Ok(())
}

/// Demo 1: Basic SurrealDB documentation pattern
async fn demo_basic_pattern() -> Result<(), Box<dyn std::error::Error>> {
	println!("\n📚 Demo 1: Basic Documentation Pattern");
	println!("--------------------------------------");

	// This follows the exact pattern from SurrealDB documentation:
	// https://surrealdb.com/docs/sdk/rust/frameworks/actix#getting-started

	// Note: In our app, DB is already connected via init_db(), but here's the pattern:

	// Connect to database (using LazyLock pattern like in our app)
	// Note: In production, we use the global DB instance from init_db()

	// Execute a simple query using our global DB instance
	let mut result = DB.query("RETURN 'Hello from SurrealDB!'").await?;
	let greeting: Option<String> = result.take(0)?;

	println!("✅ Database greeting: {:?}", greeting);
	Ok(())
}

/// Demo 2: CRUD operations with clean .await? pattern
async fn demo_crud_operations() -> Result<(), Box<dyn std::error::Error>> {
	println!("\n🔧 Demo 2: CRUD Operations");
	println!("--------------------------");

	let start = Instant::now();

	// Create a person using the clean pattern
	let person_data = PersonData {
		name: "Alice".to_string(),
		age: 30,
	};

	let created: Option<Person> = DB
		.create("person")
		.content(person_data)
		.await
		.map_err(|e| format!("Create failed: {}", e))?;
	println!("✅ Created person: {:?}", created);

	if let Some(person) = created {
		let person_id = person.id.unwrap().to_string();

		// Read the person back
		let retrieved: Vec<Person> = DB
			.select(&person_id)
			.await
			.map_err(|e| format!("Select failed: {}", e))?;
		println!("✅ Retrieved person: {:?}", retrieved.first());

		// Update the person
		let update_data = PersonData {
			name: "Alice Smith".to_string(),
			age: 31,
		};
		let updated: Vec<Person> = DB
			.update(&person_id)
			.content(update_data)
			.await
			.map_err(|e| format!("Update failed: {}", e))?;
		println!("✅ Updated person: {:?}", updated.first());

		// Delete the person
		let deleted: Vec<Person> = DB
			.delete(&person_id)
			.await
			.map_err(|e| format!("Delete failed: {}", e))?;
		println!("✅ Deleted person: {:?}", deleted.first());
	}

	let duration = start.elapsed();
	logging::performance_metric("crud_operations", duration.as_millis() as u64);

	Ok(())
}

/// Demo 3: Complex queries with error propagation
async fn demo_complex_queries() -> Result<(), Box<dyn std::error::Error>> {
	println!("\n🔍 Demo 3: Complex Queries");
	println!("---------------------------");

	// Create multiple people for querying
	let people = vec![
		PersonData {
			name: "Bob".to_string(),
			age: 25,
		},
		PersonData {
			name: "Carol".to_string(),
			age: 35,
		},
		PersonData {
			name: "Dave".to_string(),
			age: 40,
		},
	];

	let mut created_ids = Vec::new();

	// Create people using the .await? pattern
	for person_data in people {
		let created: Option<Person> = DB
			.create("person")
			.content(person_data)
			.await
			.map_err(|e| format!("Create failed: {}", e))?;
		if let Some(person) = created {
			created_ids.push(person.id.unwrap().to_string());
		}
	}

	// Query with filtering
	let mut result = DB
		.query("SELECT * FROM person WHERE age > 30")
		.await
		.map_err(|e| format!("Query failed: {}", e))?;
	let older_people: Vec<Person> = result.take(0).map_err(|e| format!("Take failed: {}", e))?;
	println!("✅ People over 30: {:?}", older_people);

	// Query with aggregation
	let mut result = DB
		.query("SELECT count() AS total, math::mean(age) AS avg_age FROM person")
		.await
		.map_err(|e| format!("Query failed: {}", e))?;
	let stats: Vec<serde_json::Value> = result.take(0).map_err(|e| format!("Take failed: {}", e))?;
	println!("✅ Person statistics: {:?}", stats);

	// Complex query with parameters
	let min_age = 25;
	let mut result = DB
		.query("SELECT * FROM person WHERE age >= $min_age ORDER BY age DESC")
		.bind(("min_age", min_age))
		.await
		.map_err(|e| format!("Query failed: {}", e))?;
	let filtered_people: Vec<Person> = result.take(0).map_err(|e| format!("Take failed: {}", e))?;
	println!("✅ People aged {} or older: {:?}", min_age, filtered_people);

	// Clean up - delete all created people
	for person_id in created_ids {
		let _: Vec<Person> = DB
			.delete(&person_id)
			.await
			.map_err(|e| format!("Delete failed: {}", e))?;
	}

	Ok(())
}

/// Demo 4: Direct DB operations with clean .await? patterns
async fn demo_direct_operations() -> Result<(), Box<dyn std::error::Error>> {
	println!("\n🛠️ Demo 4: Direct DB Operations");
	println!("--------------------------------");

	// Using direct DB operations with the clean .await? pattern
	let person_data = PersonData {
		name: "Eve".to_string(),
		age: 28,
	};

	// Create using direct DB.create().await? pattern
	let created: Option<Person> = DB
		.create("person")
		.content(person_data)
		.await
		.map_err(|e| format!("Create failed: {}", e))?;
	println!("✅ Created directly: {:?}", created);

	if let Some(person) = created {
		let person_id = person.id.unwrap().to_string();

		// Select using direct DB.select().await? pattern
		let retrieved: Vec<Person> = DB
			.select(&person_id)
			.await
			.map_err(|e| format!("Select failed: {}", e))?;
		println!("✅ Retrieved directly: {:?}", retrieved.first());

		// Update using direct DB.update().await? pattern
		let update_data = PersonData {
			name: "Eve Johnson".to_string(),
			age: 29,
		};
		let updated: Vec<Person> = DB
			.update(&person_id)
			.content(update_data)
			.await
			.map_err(|e| format!("Update failed: {}", e))?;
		println!("✅ Updated directly: {:?}", updated.first());

		// Delete using direct DB.delete().await? pattern
		let deleted: Vec<Person> = DB
			.delete(&person_id)
			.await
			.map_err(|e| format!("Delete failed: {}", e))?;
		println!("✅ Deleted directly: {:?}", deleted.first());

		// Demonstrate query with .await? pattern
		let mut result = database::query("SELECT count() FROM person")
			.await
			.map_err(|e| format!("Query failed: {}", e))?;
		let count: Option<serde_json::Value> =
			result.take(0).map_err(|e| format!("Take failed: {}", e))?;
		println!("✅ Person count after operations: {:?}", count);
	}

	Ok(())
}

/// Demo 5: Transaction-like operations with proper error handling
async fn demo_transactions() -> Result<(), Box<dyn std::error::Error>> {
	println!("\n💳 Demo 5: Transaction-like Operations");
	println!("--------------------------------------");

	// Create multiple people in a transaction-like manner
	let result = create_family().await;

	match result {
		Ok(family_ids) => {
			println!("✅ Created family with {} members", family_ids.len());

			// Query the family
			let mut result = DB
				.query("SELECT * FROM person WHERE name ~ 'Johnson'")
				.await
				.map_err(|e| format!("Query failed: {}", e))?;
			let family_members: Vec<Person> =
				result.take(0).map_err(|e| format!("Take failed: {}", e))?;
			println!("✅ Johnson family members: {:?}", family_members);

			// Clean up
			for id in family_ids {
				let _: Vec<Person> = DB
					.delete(&id)
					.await
					.map_err(|e| format!("Delete failed: {}", e))?;
			}
			println!("✅ Family cleanup completed");
		}
		Err(e) => {
			println!("❌ Failed to create family: {}", e);
			return Err(e);
		}
	}

	Ok(())
}

/// Helper function demonstrating transaction-like error handling
async fn create_family() -> Result<Vec<String>, Box<dyn std::error::Error>> {
	let family_members = vec![
		PersonData {
			name: "John Johnson".to_string(),
			age: 45,
		},
		PersonData {
			name: "Jane Johnson".to_string(),
			age: 42,
		},
		PersonData {
			name: "Jack Johnson".to_string(),
			age: 16,
		},
		PersonData {
			name: "Jill Johnson".to_string(),
			age: 14,
		},
	];

	let mut created_ids = Vec::new();

	// Create each family member - if any fails, the whole operation fails
	for member in family_members {
		let created: Option<Person> = DB
			.create("person")
			.content(member)
			.await
			.map_err(|e| format!("Create family member failed: {}", e))?;

		if let Some(person) = created {
			created_ids.push(person.id.unwrap().to_string());
		} else {
			// If creation failed, clean up already created members
			for id in &created_ids {
				let _: Vec<Person> = DB.delete(id).await.unwrap_or_default();
			}
			return Err("Failed to create family member".into());
		}
	}

	Ok(created_ids)
}
