//! Tournament entity definitions for tournament management

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

/// Full tournament record as stored in the database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tournament {
	pub id: RecordId,
	pub name: String,
	pub description: String,
	pub published: bool,
	pub created_by: RecordId,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
}

/// Data for creating a new tournament
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTournamentData {
	pub name: String,
	pub description: String,
	pub published: Option<bool>,
}

/// Data for updating an existing tournament
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTournamentData {
	pub name: Option<String>,
	pub description: Option<String>,
	pub published: Option<bool>,
}

/// Public tournament information (for listing)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicTournament {
	pub id: RecordId,
	pub name: String,
	pub description: String,
	pub published: bool,
	pub created_at: DateTime<Utc>,
}

impl From<Tournament> for PublicTournament {
	fn from(tournament: Tournament) -> Self {
		Self {
			id: tournament.id,
			name: tournament.name,
			description: tournament.description,
			published: tournament.published,
			created_at: tournament.created_at,
		}
	}
}

/// Tournament with creator information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TournamentWithCreator {
	#[serde(flatten)]
	pub tournament: Tournament,
	pub creator_username: Option<String>,
}

/// Tournament status enumeration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum TournamentStatus {
	#[default]
	Draft,
	Published,
	InProgress,
	Completed,
	Cancelled,
}

/// Tournament type enumeration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum TournamentType {
	#[default]
	SingleElimination,
	DoubleElimination,
	RoundRobin,
	Swiss,
}

#[cfg(test)]
mod tests {
	use super::*;
	use chrono::Utc;
	use surrealdb::RecordId;

	#[test]
	fn test_tournament_to_public_tournament_conversion() {
		let tournament = Tournament {
			id: RecordId::from(("tournament", "test123")),
			name: "Test Tournament".to_string(),
			description: "A test tournament".to_string(),
			published: true,
			created_by: RecordId::from(("user", "creator123")),
			created_at: Utc::now(),
			updated_at: Utc::now(),
		};

		let public_tournament: PublicTournament = tournament.clone().into();

		assert_eq!(public_tournament.id, tournament.id);
		assert_eq!(public_tournament.name, tournament.name);
		assert_eq!(public_tournament.description, tournament.description);
		assert_eq!(public_tournament.published, tournament.published);
		assert_eq!(public_tournament.created_at, tournament.created_at);
		// created_by should not be in public tournament
	}

	#[test]
	fn test_create_tournament_data_serialization() {
		let data = CreateTournamentData {
			name: "New Tournament".to_string(),
			description: "A new tournament".to_string(),
			published: Some(false),
		};

		let json = serde_json::to_string(&data).unwrap();
		let deserialized: CreateTournamentData = serde_json::from_str(&json).unwrap();

		assert_eq!(data.name, deserialized.name);
		assert_eq!(data.description, deserialized.description);
		assert_eq!(data.published, deserialized.published);
	}

	#[test]
	fn test_update_tournament_data_partial() {
		let data = UpdateTournamentData {
			name: Some("Updated Tournament".to_string()),
			description: None,
			published: Some(true),
		};

		assert!(data.name.is_some());
		assert!(data.description.is_none());
		assert!(data.published.is_some());
	}

	#[test]
	fn test_tournament_status_default() {
		let status = TournamentStatus::default();
		matches!(status, TournamentStatus::Draft);
	}

	#[test]
	fn test_tournament_type_default() {
		let tournament_type = TournamentType::default();
		matches!(tournament_type, TournamentType::SingleElimination);
	}

	#[test]
	fn test_tournament_status_serialization() {
		let status = TournamentStatus::InProgress;
		let json = serde_json::to_string(&status).unwrap();
		assert_eq!(json, "\"in_progress\"");

		let deserialized: TournamentStatus = serde_json::from_str(&json).unwrap();
		matches!(deserialized, TournamentStatus::InProgress);
	}

	#[test]
	fn test_tournament_type_serialization() {
		let tournament_type = TournamentType::DoubleElimination;
		let json = serde_json::to_string(&tournament_type).unwrap();
		assert_eq!(json, "\"double_elimination\"");

		let deserialized: TournamentType = serde_json::from_str(&json).unwrap();
		matches!(deserialized, TournamentType::DoubleElimination);
	}
}
