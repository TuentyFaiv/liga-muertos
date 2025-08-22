import { describe, expect, it } from "vitest";

// Mock tournament types and functions for testing
interface Tournament {
	id?: string;
	name: string;
	participants: string[];
	status: "draft" | "active" | "completed";
	createdAt?: Date;
}

interface CreateTournamentRequest {
	name: string;
	participants: string[];
}

interface ValidationResult {
	isValid: boolean;
	errors: string[];
}

// Tournament utility functions to be tested
function createTournament(request: CreateTournamentRequest): Tournament {
	return {
		id: crypto.randomUUID(),
		name: request.name,
		participants: [...request.participants],
		status: "draft",
		createdAt: new Date(),
	};
}

function validateTournament(tournament: Tournament): ValidationResult {
	const errors: string[] = [];

	if (!tournament.name || tournament.name.trim().length === 0) {
		errors.push("Tournament name is required");
	}

	if (tournament.name && tournament.name.length > 100) {
		errors.push("Tournament name must be less than 100 characters");
	}

	if (tournament.participants.length < 2) {
		errors.push("Tournament must have at least 2 participants");
	}

	if (tournament.participants.length > 64) {
		errors.push("Tournament cannot have more than 64 participants");
	}

	// Check for duplicate participants
	const uniqueParticipants = new Set(tournament.participants);
	if (uniqueParticipants.size !== tournament.participants.length) {
		errors.push("Tournament participants must be unique");
	}

	return {
		isValid: errors.length === 0,
		errors,
	};
}

function generateMatches(
	participants: string[]
): Array<{ id: string; player1: string; player2: string; winner?: string }> {
	const matches = [];
	const shuffledParticipants = [...participants].sort(() => Math.random() - 0.5);

	for (let i = 0; i < shuffledParticipants.length; i += 2) {
		if (i + 1 < shuffledParticipants.length) {
			matches.push({
				id: crypto.randomUUID(),
				player1: shuffledParticipants[i],
				player2: shuffledParticipants[i + 1],
			});
		}
	}

	return matches;
}

// Test suites
describe("Tournament Logic", () => {
	describe("createTournament", () => {
		it("should create a valid tournament with provided data", () => {
			const request = {
				name: "La Liga Test Tournament",
				participants: ["Player1", "Player2", "Player3", "Player4"],
			};

			const tournament = createTournament(request);

			expect(tournament).toBeDefined();
			expect(tournament.id).toBeTruthy();
			expect(tournament.name).toBe("La Liga Test Tournament");
			expect(tournament.participants).toEqual(["Player1", "Player2", "Player3", "Player4"]);
			expect(tournament.status).toBe("draft");
			expect(tournament.createdAt).toBeInstanceOf(Date);
		});

		it("should create unique IDs for different tournaments", () => {
			const request1 = { name: "Tournament 1", participants: ["A", "B"] };
			const request2 = { name: "Tournament 2", participants: ["C", "D"] };

			const tournament1 = createTournament(request1);
			const tournament2 = createTournament(request2);

			expect(tournament1.id).not.toBe(tournament2.id);
		});

		it("should not mutate the original participants array", () => {
			const originalParticipants = ["Alice", "Bob", "Charlie"];
			const request = {
				name: "Test Tournament",
				participants: originalParticipants,
			};

			const tournament = createTournament(request);
			tournament.participants.push("David");

			expect(originalParticipants).toHaveLength(3);
			expect(originalParticipants).not.toContain("David");
		});
	});

	describe("validateTournament", () => {
		it("should validate a correct tournament", () => {
			const tournament: Tournament = {
				name: "Valid Tournament",
				participants: ["Alice", "Bob", "Charlie", "David"],
				status: "draft",
			};

			const result = validateTournament(tournament);

			expect(result.isValid).toBe(true);
			expect(result.errors).toHaveLength(0);
		});

		it("should reject tournament with empty name", () => {
			const tournament: Tournament = {
				name: "",
				participants: ["Alice", "Bob"],
				status: "draft",
			};

			const result = validateTournament(tournament);

			expect(result.isValid).toBe(false);
			expect(result.errors).toContain("Tournament name is required");
		});

		it("should reject tournament with whitespace-only name", () => {
			const tournament: Tournament = {
				name: "   ",
				participants: ["Alice", "Bob"],
				status: "draft",
			};

			const result = validateTournament(tournament);

			expect(result.isValid).toBe(false);
			expect(result.errors).toContain("Tournament name is required");
		});

		it("should reject tournament with too long name", () => {
			const tournament: Tournament = {
				name: "A".repeat(101),
				participants: ["Alice", "Bob"],
				status: "draft",
			};

			const result = validateTournament(tournament);

			expect(result.isValid).toBe(false);
			expect(result.errors).toContain("Tournament name must be less than 100 characters");
		});

		it("should reject tournament with insufficient participants", () => {
			const tournament: Tournament = {
				name: "Solo Tournament",
				participants: ["Alice"],
				status: "draft",
			};

			const result = validateTournament(tournament);

			expect(result.isValid).toBe(false);
			expect(result.errors).toContain("Tournament must have at least 2 participants");
		});

		it("should reject tournament with too many participants", () => {
			const participants = Array.from({ length: 65 }, (_, i) => `Player${i + 1}`);
			const tournament: Tournament = {
				name: "Massive Tournament",
				participants,
				status: "draft",
			};

			const result = validateTournament(tournament);

			expect(result.isValid).toBe(false);
			expect(result.errors).toContain("Tournament cannot have more than 64 participants");
		});

		it("should reject tournament with duplicate participants", () => {
			const tournament: Tournament = {
				name: "Duplicate Tournament",
				participants: ["Alice", "Bob", "Alice", "Charlie"],
				status: "draft",
			};

			const result = validateTournament(tournament);

			expect(result.isValid).toBe(false);
			expect(result.errors).toContain("Tournament participants must be unique");
		});

		it("should return multiple errors for multiple issues", () => {
			const tournament: Tournament = {
				name: "",
				participants: ["Alice"], // Only one participant
				status: "draft",
			};

			const result = validateTournament(tournament);

			expect(result.isValid).toBe(false);
			expect(result.errors).toHaveLength(2);
			expect(result.errors).toContain("Tournament name is required");
			expect(result.errors).toContain("Tournament must have at least 2 participants");
		});
	});

	describe("generateMatches", () => {
		it("should generate correct number of matches for even participants", () => {
			const participants = ["Alice", "Bob", "Charlie", "David"];
			const matches = generateMatches(participants);

			expect(matches).toHaveLength(2);
			expect(matches[0]).toHaveProperty("id");
			expect(matches[0]).toHaveProperty("player1");
			expect(matches[0]).toHaveProperty("player2");
			expect(matches[0].winner).toBeUndefined();
		});

		it("should generate correct number of matches for odd participants", () => {
			const participants = ["Alice", "Bob", "Charlie", "David", "Eve"];
			const matches = generateMatches(participants);

			// With 5 participants, we should get 2 matches (4 participants paired, 1 bye)
			expect(matches).toHaveLength(2);
		});

		it("should include all participants in matches", () => {
			const participants = ["Alice", "Bob", "Charlie", "David"];
			const matches = generateMatches(participants);

			const playersInMatches = new Set([
				...matches.map((match) => match.player1),
				...matches.map((match) => match.player2),
			]);

			for (const participant of participants) {
				expect(playersInMatches.has(participant)).toBe(true);
			}
		});

		it("should generate unique match IDs", () => {
			const participants = ["Alice", "Bob", "Charlie", "David"];
			const matches = generateMatches(participants);

			const matchIds = matches.map((match) => match.id);
			const uniqueIds = new Set(matchIds);

			expect(uniqueIds.size).toBe(matches.length);
		});

		it("should handle minimum participants", () => {
			const participants = ["Alice", "Bob"];
			const matches = generateMatches(participants);

			expect(matches).toHaveLength(1);
			expect(matches[0].player1).toBe("Alice");
			expect(matches[0].player2).toBe("Bob");
		});

		it("should not pair same player against themselves", () => {
			const participants = ["Alice", "Bob", "Charlie", "David"];
			const matches = generateMatches(participants);

			for (const match of matches) {
				expect(match.player1).not.toBe(match.player2);
			}
		});
	});
});
