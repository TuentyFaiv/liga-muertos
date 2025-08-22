# Testing Guide for La Liga de los Muertos

This guide explains how to run tests, write new tests, and understand our testing philosophy for both frontend and backend components.

## 🎯 Testing Philosophy

We believe in **confidence through testing**. Our testing strategy focuses on:

- **Reliability**: Contributors can confidently make changes knowing tests will catch regressions
- **Developer Experience**: Fast, reliable tests that provide clear feedback
- **Realistic Testing**: Tests that closely mirror real user interactions
- **Maintainability**: Tests that are easy to read, write, and maintain

## 🏗️ Testing Architecture

### Frontend Testing Stack
- **Unit Tests**: Vitest with browser environment
- **Component Tests**: Svelte Testing Library patterns
- **Integration Tests**: API and service integration
- **E2E Tests**: Playwright for full user workflows
- **Visual Tests**: Storybook for component development

### Backend Testing Stack
- **Unit Tests**: Native Rust testing with `cargo test`
- **Integration Tests**: Actix Web test utilities
- **API Tests**: HTTP endpoint testing
- **Mock Services**: Wiremock for external service mocking
- **Database Tests**: In-memory and fixture-based testing

## 🚀 Quick Start

### Running All Tests

```bash
# Frontend tests
cd front
bun run test          # All tests (unit + e2e)
bun run test:unit     # Unit tests only
bun run test:e2e      # E2E tests only

# Backend tests
cd back
cargo test            # All Rust tests
cargo test --verbose  # With detailed output
```

### Running Tests with Coverage

```bash
# Frontend coverage
cd front
bun run test:unit -- --coverage

# Backend coverage
cd back
cargo llvm-cov --html  # Generates HTML report
cargo llvm-cov --lcov  # Generates LCOV format
```

## 🎭 Frontend Testing

### Unit Testing with Vitest

Our unit tests use Vitest in browser mode for the most realistic environment.

**Location**: `src/**/*.{test,spec}.{js,ts}`

```typescript
// src/logic/tournament.test.ts
import { describe, it, expect } from 'vitest';
import { createTournament, validateTournament } from './tournament.js';

describe('Tournament Logic', () => {
  it('should create a valid tournament', () => {
    const tournament = createTournament({
      name: 'Test Tournament',
      participants: ['Player1', 'Player2']
    });

    expect(tournament).toBeDefined();
    expect(tournament.name).toBe('Test Tournament');
    expect(tournament.participants).toHaveLength(2);
  });

  it('should validate tournament structure', () => {
    const tournament = createTournament({ name: 'Test', participants: [] });
    const result = validateTournament(tournament);

    expect(result.isValid).toBe(false);
    expect(result.errors).toContain('Tournament must have at least 2 participants');
  });
});
```

### Component Testing

For Svelte components, we test behavior and integration:

```typescript
// src/ui/TournamentBracket.svelte.test.ts
import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import { userEvent } from '@testing-library/user-event';
import TournamentBracket from './TournamentBracket.svelte';

describe('TournamentBracket Component', () => {
  it('renders tournament matches', () => {
    const mockTournament = {
      name: 'Test Tournament',
      matches: [
        { id: '1', player1: 'Alice', player2: 'Bob', winner: null }
      ]
    };

    render(TournamentBracket, { tournament: mockTournament });

    expect(screen.getByText('Alice')).toBeInTheDocument();
    expect(screen.getByText('Bob')).toBeInTheDocument();
  });

  it('handles match winner selection', async () => {
    const mockTournament = { /* ... */ };
    const user = userEvent.setup();

    render(TournamentBracket, { tournament: mockTournament });

    const aliceButton = screen.getByRole('button', { name: /alice/i });
    await user.click(aliceButton);

    expect(screen.getByText('Winner: Alice')).toBeInTheDocument();
  });
});
```

### E2E Testing with Playwright

**Location**: `e2e/*.test.ts`

```typescript
// e2e/tournament-flow.test.ts
import { test, expect } from '@playwright/test';

test.describe('Tournament Management', () => {
  test('creates and manages a tournament', async ({ page }) => {
    await page.goto('/');

    // Navigate to tournament creation
    await page.click('text=Create Tournament');
    await expect(page.locator('h1')).toContainText('New Tournament');

    // Fill tournament details
    await page.fill('[data-testid="tournament-name"]', 'My Test Tournament');
    await page.click('[data-testid="add-participant"]');
    await page.fill('[data-testid="participant-1"]', 'Alice');
    await page.fill('[data-testid="participant-2"]', 'Bob');

    // Create tournament
    await page.click('[data-testid="create-tournament"]');

    // Verify tournament was created
    await expect(page.locator('[data-testid="tournament-title"]'))
      .toContainText('My Test Tournament');
    await expect(page.locator('[data-testid="participant"]'))
      .toHaveCount(2);
  });

  test('handles authentication flow', async ({ page }) => {
    await page.goto('/dashboard');

    // Should redirect to login
    await expect(page).toHaveURL(/.*login/);

    // Mock Clerk authentication for testing
    await page.route('**/api/auth/**', (route) => {
      route.fulfill({
        status: 200,
        body: JSON.stringify({ user: { id: '1', username: 'testuser' } })
      });
    });

    await page.click('[data-testid="login-button"]');
    await expect(page).toHaveURL(/.*dashboard/);
  });
});
```

## 🦀 Backend Testing

### Unit Tests

**Location**: `src/**/*.rs` (with `#[cfg(test)]` modules)

```rust
// src/services/tournament.rs
#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Tournament;

    #[tokio::test]
    async fn test_create_tournament() {
        let tournament_data = CreateTournamentRequest {
            name: "Test Tournament".to_string(),
            participants: vec!["Alice".to_string(), "Bob".to_string()],
        };

        let result = create_tournament(tournament_data).await;

        assert!(result.is_ok());
        let tournament = result.unwrap();
        assert_eq!(tournament.name, "Test Tournament");
        assert_eq!(tournament.participants.len(), 2);
    }

    #[tokio::test]
    async fn test_tournament_validation() {
        let invalid_data = CreateTournamentRequest {
            name: "".to_string(), // Invalid: empty name
            participants: vec!["Alice".to_string()], // Invalid: not enough participants
        };

        let result = create_tournament(invalid_data).await;

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.to_string().contains("Tournament name cannot be empty"));
    }
}
```

### Integration Tests

**Location**: `tests/integration/`

```rust
// tests/integration/tournament_api.rs
use actix_web::{test, web, App};
use liga_muertos_back::{configure_app, AppState};

#[actix_web::test]
async fn test_create_tournament_endpoint() {
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::new_test()))
            .configure(configure_app)
    ).await;

    let tournament_data = serde_json::json!({
        "name": "Test Tournament",
        "participants": ["Alice", "Bob", "Charlie", "David"]
    });

    let req = test::TestRequest::post()
        .uri("/api/tournaments")
        .set_json(&tournament_data)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["name"], "Test Tournament");
    assert_eq!(body["participants"].as_array().unwrap().len(), 4);
}
```

### Database Testing

```rust
// tests/integration/database.rs
use surrealdb::{Surreal, engine::local::Mem};

#[tokio::test]
async fn test_tournament_persistence() {
    // Use in-memory database for testing
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let tournament = Tournament {
        id: None,
        name: "Test Tournament".to_string(),
        participants: vec!["Alice".to_string(), "Bob".to_string()],
        created_at: chrono::Utc::now(),
    };

    // Test creation
    let created: Option<Tournament> = db
        .create("tournament")
        .content(tournament.clone())
        .await
        .unwrap();

    assert!(created.is_some());
    let saved_tournament = created.unwrap();
    assert!(saved_tournament.id.is_some());
    assert_eq!(saved_tournament.name, tournament.name);
}
```

## 🎨 Storybook Component Testing

We use Storybook for visual component development and testing.

```typescript
// src/stories/TournamentBracket.stories.ts
import type { Meta, StoryObj } from '@storybook/svelte';
import TournamentBracket from '../ui/TournamentBracket.svelte';

const meta = {
  title: 'Components/TournamentBracket',
  component: TournamentBracket,
  parameters: {
    layout: 'fullscreen',
  },
  tags: ['autodocs'],
  argTypes: {
    tournament: {
      control: 'object',
      description: 'Tournament data with matches and participants'
    },
  },
} satisfies Meta<TournamentBracket>;

export default meta;
type Story = StoryObj<typeof meta>;

export const SingleElimination: Story = {
  args: {
    tournament: {
      name: 'Championship 2024',
      format: 'single-elimination',
      matches: [
        { id: '1', player1: 'Alice', player2: 'Bob', winner: null },
        { id: '2', player1: 'Charlie', player2: 'David', winner: null },
      ]
    },
  },
};

export const WithWinners: Story = {
  args: {
    tournament: {
      name: 'Completed Tournament',
      format: 'single-elimination',
      matches: [
        { id: '1', player1: 'Alice', player2: 'Bob', winner: 'Alice' },
        { id: '2', player1: 'Charlie', player2: 'David', winner: 'David' },
        { id: '3', player1: 'Alice', player2: 'David', winner: 'Alice' },
      ]
    },
  },
};
```

## 🔄 Continuous Integration

### GitHub Actions Workflows

Our CI pipeline runs multiple test suites:

1. **Pre-commit Quality Gates** (`pre-commit.yml`):
   - Linting and formatting checks
   - Type checking
   - Commit message validation
   - File size and TODO checks

2. **Main CI Pipeline** (`ci.yml`):
   - Frontend unit and E2E tests
   - Backend tests with all features
   - Storybook build verification
   - Security audits
   - Documentation link checking

3. **Coverage Reporting** (`coverage.yml`):
   - Test coverage collection
   - Coverage threshold enforcement (70% minimum)
   - Coverage reports in PR comments
   - Codecov integration

### Running CI Locally

```bash
# Simulate the CI environment locally
cd front
bun run lint && bun run check && bun run test && bun run build

cd ../back
cargo fmt --check && cargo clippy -- -D warnings && cargo test && cargo build --release
```

## 📊 Coverage Requirements

### Minimum Coverage Thresholds

- **Frontend**: 70% line coverage
- **Backend**: 70% line coverage
- **New code**: 80% line coverage (aspirational)

### Coverage Reports

Coverage reports are generated automatically and include:
- Line-by-line coverage highlighting
- Function and branch coverage
- Uncovered code identification
- Historical coverage trends

## 🚨 Testing Best Practices

### Writing Good Tests

1. **Use descriptive test names**:
   ```rust
   // ❌ Bad
   #[test]
   fn test_tournament() { ... }

   // ✅ Good
   #[test]
   fn test_create_tournament_with_valid_participants_succeeds() { ... }
   ```

2. **Follow AAA pattern** (Arrange, Act, Assert):
   ```typescript
   it('should calculate tournament winner', () => {
     // Arrange
     const matches = createTestMatches();
     const tournament = new Tournament(matches);

     // Act
     const winner = tournament.calculateWinner();

     // Assert
     expect(winner).toBe('Alice');
   });
   ```

3. **Test behavior, not implementation**:
   ```typescript
   // ❌ Testing implementation details
   expect(component.internal_state.counter).toBe(5);

   // ✅ Testing behavior
   expect(screen.getByText('Count: 5')).toBeInTheDocument();
   ```

4. **Use realistic test data**:
   ```rust
   // Use factories or fixtures for consistent test data
   fn create_test_tournament() -> Tournament {
       Tournament {
           name: "World Championship".to_string(),
           participants: vec![
               "Alice".to_string(),
               "Bob".to_string(),
               "Charlie".to_string(),
               "David".to_string(),
           ],
           format: TournamentFormat::SingleElimination,
           created_at: Utc::now(),
       }
   }
   ```

### Test Organization

- Keep tests close to the code they test
- Use descriptive `describe`/`mod` blocks for grouping
- Separate unit, integration, and E2E tests clearly
- Use shared fixtures and utilities to avoid duplication

### Performance Considerations

- **Frontend**: Use `vi.mock()` for expensive operations
- **Backend**: Use in-memory databases for fast tests
- **E2E**: Run in parallel when possible, use page objects for reusability
- **CI**: Cache dependencies and use matrix builds for efficiency

## 🛠️ Debugging Tests

### Frontend Test Debugging

```typescript
// Add debug output
import { screen } from '@testing-library/svelte';

test('debugging example', () => {
  render(MyComponent);

  // See what's rendered
  screen.debug();

  // Check specific elements
  console.log(screen.getByTestId('my-element').textContent);
});
```

### Backend Test Debugging

```rust
// Use test output
#[test]
fn debug_test() {
    let result = my_function();
    dbg!(&result); // Print debug info
    println!("Result: {:?}", result); // Custom output

    assert_eq!(result, expected_value);
}
```

### E2E Test Debugging

```typescript
// Playwright debugging
test('debug e2e test', async ({ page }) => {
  // Slow down actions
  await page.pause();

  // Take screenshots
  await page.screenshot({ path: 'debug.png' });

  // Enable tracing
  await context.tracing.start({ screenshots: true, snapshots: true });
  // ... test actions ...
  await context.tracing.stop({ path: 'trace.zip' });
});
```

## 📚 Resources

### Documentation
- [Vitest Documentation](https://vitest.dev/)
- [Playwright Documentation](https://playwright.dev/)
- [Rust Testing Guide](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Actix Web Testing](https://actix.rs/docs/testing/)

### Tools
- [Storybook](https://storybook.js.org/) - Component development and testing
- [Codecov](https://codecov.io/) - Coverage reporting and analysis
- [cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov) - Rust coverage

### Getting Help

If you need help with testing:

1. Check existing tests for patterns and examples
2. Review the testing documentation above
3. Ask questions in GitHub Discussions
4. Create issues for testing infrastructure improvements

Remember: **Good tests are worth the investment**. They save time, prevent bugs, and make everyone more confident in the codebase! 🚀
