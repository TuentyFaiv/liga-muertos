//! Participant entity definitions for tournament participation

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

/// Full participant record as stored in the database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Participant {
	pub id: RecordId,
	pub tournament: RecordId,
	pub user_id: RecordId,
	pub joined_at: DateTime<Utc>,
}

/// Data for joining a tournament
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JoinTournamentData {
	pub tournament: RecordId,
}

/// Data for creating a participant (admin use)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateParticipantData {
	pub tournament: RecordId,
	pub user_id: RecordId,
}

/// Participant with user information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParticipantWithUser {
	pub id: RecordId,
	pub tournament: RecordId,
	pub user_id: RecordId,
	pub username: String,
	pub joined_at: DateTime<Utc>,
}

/// Participant with tournament information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParticipantWithTournament {
	pub id: RecordId,
	pub tournament: RecordId,
	pub tournament_name: String,
	pub user_id: RecordId,
	pub joined_at: DateTime<Utc>,
}

/// Full participant information with both user and tournament data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullParticipantInfo {
	pub id: RecordId,
	pub tournament: RecordId,
	pub tournament_name: String,
	pub user_id: RecordId,
	pub username: String,
	pub joined_at: DateTime<Utc>,
}

/// Participant status enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParticipantStatus {
	Registered,
	Active,
	Eliminated,
	Withdrawn,
	Disqualified,
}

impl Default for ParticipantStatus {
	fn default() -> Self {
		ParticipantStatus::Registered
	}
}

/// Tournament participation statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParticipantStats {
	pub participant_id: RecordId,
	pub matches_played: u32,
	pub matches_won: u32,
	pub matches_lost: u32,
	pub win_rate: f64,
}

impl ParticipantStats {
	pub fn new(participant_id: RecordId) -> Self {
		Self {
			participant_id,
			matches_played: 0,
			matches_won: 0,
			matches_lost: 0,
			win_rate: 0.0,
		}
	}

	pub fn calculate_win_rate(&mut self) {
		if self.matches_played > 0 {
			self.win_rate = (self.matches_won as f64) / (self.matches_played as f64);
		} else {
			self.win_rate = 0.0;
		}
	}

	pub fn add_match_result(&mut self, won: bool) {
		self.matches_played += 1;
		if won {
			self.matches_won += 1;
		} else {
			self.matches_lost += 1;
		}
		self.calculate_win_rate();
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use chrono::Utc;
	use surrealdb::RecordId;

	#[test]
	fn test_participant_creation() {
		let participant = Participant {
			id: RecordId::from(("participant", "test123")),
			tournament: RecordId::from(("tournament", "tourney123")),
			user_id: RecordId::from(("user", "user123")),
			joined_at: Utc::now(),
		};

		assert_eq!(participant.id, RecordId::from(("participant", "test123")));
		assert_eq!(
			participant.tournament,
			RecordId::from(("tournament", "tourney123"))
		);
		assert_eq!(participant.user_id, RecordId::from(("user", "user123")));
	}

	#[test]
	fn test_join_tournament_data_serialization() {
		let data = JoinTournamentData {
			tournament: RecordId::from(("tournament", "tourney123")),
		};

		let json = serde_json::to_string(&data).unwrap();
		let deserialized: JoinTournamentData = serde_json::from_str(&json).unwrap();

		assert_eq!(data.tournament, deserialized.tournament);
	}

	#[test]
	fn test_participant_status_default() {
		let status = ParticipantStatus::default();
		matches!(status, ParticipantStatus::Registered);
	}

	#[test]
	fn test_participant_status_serialization() {
		let status = ParticipantStatus::Active;
		let json = serde_json::to_string(&status).unwrap();
		assert_eq!(json, "\"active\"");

		let deserialized: ParticipantStatus = serde_json::from_str(&json).unwrap();
		matches!(deserialized, ParticipantStatus::Active);
	}

	#[test]
	fn test_participant_stats_new() {
		let participant_id = RecordId::from(("participant", "test123"));
		let stats = ParticipantStats::new(participant_id.clone());

		assert_eq!(stats.participant_id, participant_id);
		assert_eq!(stats.matches_played, 0);
		assert_eq!(stats.matches_won, 0);
		assert_eq!(stats.matches_lost, 0);
		assert_eq!(stats.win_rate, 0.0);
	}

	#[test]
	fn test_participant_stats_add_match_result() {
		let participant_id = RecordId::from(("participant", "test123"));
		let mut stats = ParticipantStats::new(participant_id);

		// Add a win
		stats.add_match_result(true);
		assert_eq!(stats.matches_played, 1);
		assert_eq!(stats.matches_won, 1);
		assert_eq!(stats.matches_lost, 0);
		assert_eq!(stats.win_rate, 1.0);

		// Add a loss
		stats.add_match_result(false);
		assert_eq!(stats.matches_played, 2);
		assert_eq!(stats.matches_won, 1);
		assert_eq!(stats.matches_lost, 1);
		assert_eq!(stats.win_rate, 0.5);

		// Add another win
		stats.add_match_result(true);
		assert_eq!(stats.matches_played, 3);
		assert_eq!(stats.matches_won, 2);
		assert_eq!(stats.matches_lost, 1);
		assert!((stats.win_rate - (2.0 / 3.0)).abs() < f64::EPSILON);
	}

	#[test]
	fn test_participant_stats_calculate_win_rate() {
		let participant_id = RecordId::from(("participant", "test123"));
		let mut stats = ParticipantStats::new(participant_id);

		// Manually set some values
		stats.matches_played = 4;
		stats.matches_won = 3;
		stats.matches_lost = 1;

		stats.calculate_win_rate();
		assert_eq!(stats.win_rate, 0.75);
	}

	#[test]
	fn test_create_participant_data() {
		let data = CreateParticipantData {
			tournament: RecordId::from(("tournament", "tourney123")),
			user_id: RecordId::from(("user", "user123")),
		};

		assert_eq!(
			data.tournament,
			RecordId::from(("tournament", "tourney123"))
		);
		assert_eq!(data.user_id, RecordId::from(("user", "user123")));
	}
}
