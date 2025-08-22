# Contributing to La Liga de los Muertos

Thank you for your interest in contributing to La Liga de los Muertos! This document provides guidelines and information for contributors to help maintain code quality and streamline the development process.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Newcomer Guide](#newcomer-guide)
- [Development Workflow](#development-workflow)
- [Git Flow and Branching](#git-flow-and-branching)
- [Commit Message Guidelines](#commit-message-guidelines)
- [Pull Request Process](#pull-request-process)
- [Issue Guidelines](#issue-guidelines)
- [GitHub Labels System](#github-labels-system)
- [Development Setup](#development-setup)
- [Code Style and Standards](#code-style-and-standards)
- [Testing Guidelines](#testing-guidelines)
- [Automated Testing](#automated-testing)
- [Documentation](#documentation)
- [Getting Help](#getting-help)

## Code of Conduct

By participating in this project, you agree to abide by our [Code of Conduct](./CODE_OF_CONDUCT.md). We are committed to providing a welcoming and inclusive environment for all contributors.

Please read the full Code of Conduct to understand the standards we expect from all community members, contributors, and leaders. It outlines our commitment to creating a harassment-free experience for everyone, regardless of background or identity.

## Getting Started

Before you start contributing, please:

1. **Fork the repository** on GitHub
2. **Set up your development environment** (see [Development Setup](#development-setup))
3. **Read through the documentation** to understand the project structure
4. **Look for issues** labeled `good first issue` or `help wanted` if you're new to the project
5. **Check our [Newcomer Guide](./.github/NEWCOMERS.md)** if this is your first contribution

## Newcomer Guide

👋 **New to the project?** We've created a comprehensive guide just for you!

**📖 [Read the Newcomer Guide](./.github/NEWCOMERS.md)** - This guide covers:
- How to choose your first issue
- Step-by-step contribution process
- Understanding our tech stack
- Common newcomer mistakes and how to avoid them
- Learning resources and getting help

**🎯 Quick Links for Newcomers:**
- [Good First Issues](https://github.com/TuentyFaiv/liga-muertos/labels/good%20first%20issue) - Perfect for your first contribution (1-3 hours)
- [Beginner Friendly Issues](https://github.com/TuentyFaiv/liga-muertos/labels/beginner%20friendly) - Good for those with some experience (3-6 hours)
- [Help Wanted Issues](https://github.com/TuentyFaiv/liga-muertos/labels/help%20wanted) - Issues where we especially need help

## Development Workflow

We use a simplified Git Flow branching model designed for collaborative development while maintaining code quality and stability.

### Git Flow and Branching

#### Main Branches

- **`main`**: Production-ready codebase reflecting the latest stable release
- **`develop`**: Integration branch for development changes and new features

#### Branch Types and Naming Conventions

Create new branches from the latest `develop` or `main` branch using these naming conventions:

| Branch Type | Prefix | Example | Purpose |
|-------------|--------|---------|---------|
| Feature | `feat/` | `feat/tournament-brackets` | New feature implementations |
| Bug Fix | `fix/` | `fix/user-authentication` | Bug fixes and corrections |
| Styling | `styles/` | `styles/responsive-navbar` | UI/UX and styling improvements |
| Refactor | `refactor/` | `refactor/api-endpoints` | Code refactoring and optimization |

#### Branch Workflow

1. **Create a branch** from `develop` (or `main` for hotfixes):
   ```bash
   git checkout develop
   git pull origin develop
   git checkout -b feat/your-feature-name
   ```

2. **Make your changes** following our coding standards
3. **Commit your changes** using our commit message format
4. **Push your branch** to your fork
5. **Create a Pull Request** using our PR template

### Commit Message Guidelines

All commit messages **must** include both a gitmoji and a prefix to maintain consistency and clarity.

#### Format

```
<gitmoji> <PREFIX>: <short description>

<detailed description (optional)>
```

#### Prefixes and Gitmojis

| Gitmoji | Prefix | Usage |
|---------|--------|--------|
| ✨ | `FEAT` | New feature implementations |
| 🐛 | `FIX` | Bug fixes and error corrections |
| 💄 | `STYLES` | Styling, UI/UX, and responsiveness changes |
| 🔧 | `CHORE` | Configuration files and build process changes |
| 📝 | `DOCS` | Documentation updates and improvements |
| ♻️ | `REFACTOR` | Code refactoring without functional changes |
| 🧪 | `TEST` | Adding or updating tests |
| 🚀 | `DEPLOY` | Deployment-related changes |
| 🔒 | `SECURITY` | Security-related improvements |

#### Examples

```bash
# Good commit messages
git commit -m "✨ FEAT: Add tournament bracket visualization" -m "Implemented interactive SVG-based tournament bracket with real-time updates and responsive design"

git commit -m "🐛 FIX: Resolve authentication redirect loop" -m "Fixed infinite redirect issue in Clerk authentication flow when users have incomplete profiles"

git commit -m "💄 STYLES: Improve mobile navigation experience" -m "Updated navbar to use hamburger menu on mobile devices with smooth animations"

# Bad commit messages (avoid these)
git commit -m "fix bug"
git commit -m "update code"
git commit -m "changes"
```

### Pull Request Process

#### Before Creating a PR

1. **Ensure your branch is up to date** with the target branch
2. **Run all tests** and ensure they pass
3. **Check code style** and fix any linting issues
4. **Update documentation** if necessary
5. **Test your changes** thoroughly

#### Creating the PR

1. **Push your branch** to your fork on GitHub
2. **Create a new Pull Request** from your fork to the main repository
3. **Use the PR template** and fill in all required sections
4. **Add appropriate labels** (e.g., `frontend`, `backend`, `bug`, `feature`)
5. **Assign yourself** as the assignee
6. **Request reviews** from at least two team members
7. **Link related issues** using keywords like "Closes #123"

#### PR Requirements

- [ ] All tests pass
- [ ] Code follows style guidelines
- [ ] Documentation is updated
- [ ] PR template is completely filled out
- [ ] At least 2 reviewers assigned
- [ ] Related issues are linked
- [ ] No merge conflicts

#### Review Process

1. **Address feedback** promptly and professionally
2. **Update your branch** with requested changes
3. **Respond to comments** to facilitate discussion
4. **Request re-review** after making changes

## Issue Guidelines

### Before Creating an Issue

1. **Search existing issues** to avoid duplicates
2. **Check the documentation** for answers to your question
3. **Reproduce the issue** if it's a bug report
4. **Gather relevant information** (browser, OS, versions, etc.)

### Issue Types

We provide several issue templates:

- **🐛 Bug Report**: For reporting bugs or unexpected behavior
- **✨ Feature Request**: For suggesting new features or enhancements
- **📝 Documentation**: For documentation improvements
- **❓ Question/Discussion**: For questions or project discussions

## GitHub Labels System

We use a comprehensive labeling system to help organize issues and guide contributors effectively.

**📋 [Full Labels Guide](./.github/LABELS.md)** - Complete documentation of our labeling system

### Quick Label Reference

#### 🌟 For Newcomers
| Label | Description | Time Estimate |
|-------|-------------|---------------|
| `good first issue` | Perfect for newcomers | 1-3 hours |
| `beginner friendly` | Suitable for beginners | 3-6 hours |
| `help wanted` | Extra attention needed | Varies |

#### 🏷️ Component Labels
| Label | Description |
|-------|-------------|
| `component: frontend` | SvelteKit frontend related |
| `component: backend` | Rust backend related |
| `component: database` | SurrealDB related |
| `component: auth` | Authentication (Clerk/Twitch) |
| `component: api` | API related |

#### 📝 Type Labels
| Label | Description |
|-------|-------------|
| `type: bug` | Something isn't working |
| `type: feature` | New feature or enhancement |
| `type: documentation` | Documentation improvements |
| `type: refactor` | Code refactoring |
| `type: chore` | Maintenance tasks |

## Development Setup

### Prerequisites

- **Node.js** (v22 or newer)
- **Bun** package manager
- **Rust** (latest stable version)
- **Git** with SSH keys configured
- **SurrealDB Cloud** account
- **Clerk** account for authentication

### Frontend Setup

```bash
# Clone the repository
git clone git@github.com:TuentyFaiv/liga-muertos.git
cd liga-muertos/front

# Install dependencies
bun install

# Start development server
bun run dev
```

### Backend Setup

```bash
# Navigate to backend directory
cd liga-muertos/back

# Build the project
cargo build

# Run development server
cargo run
```

### Environment Configuration

Create appropriate `.env` files in both frontend and backend directories with required environment variables. Refer to the respective README files for specific configuration details.

## Code Style and Standards

### General Guidelines

- **Write clear, readable code** with meaningful variable and function names
- **Add comments** for complex logic and business rules
- **Follow established patterns** within the codebase
- **Keep functions small** and focused on a single responsibility
- **Use consistent formatting** as enforced by our linting tools

### Frontend (SvelteKit)

- Use **Biome** for code formatting and linting
- Follow **SvelteKit conventions** for file structure and naming
- Use **TypeScript** for type safety
- Write **accessible HTML** following WCAG guidelines
- Use **TailwindCSS** for styling with consistent design tokens

### Backend (Rust)

- Follow **Rust idioms** and best practices
- Use **cargo fmt** for code formatting
- Run **cargo clippy** for additional linting
- Write **comprehensive error handling**
- Document **public APIs** with doc comments

### Database

- Follow **SurrealDB schema conventions**
- Write **optimized queries** with proper indexing
- Document **schema changes** and migrations

## Testing Guidelines

We have comprehensive automated testing to ensure code quality and catch regressions early. All contributors should run tests before submitting PRs.

### Test Coverage Requirements

- **Minimum Coverage**: 70% line coverage for both frontend and backend
- **New Code**: Aim for 80%+ coverage on new features
- **Bug Fixes**: Must include regression tests
- **Critical Features**: Require comprehensive test coverage

### Frontend Testing

**Test Types:**
- **Unit Tests**: Vitest with browser environment for logic and utilities
- **Component Tests**: Svelte Testing Library for component behavior
- **E2E Tests**: Playwright for full user workflows
- **Visual Tests**: Storybook for component development and visual regression

**Running Frontend Tests:**
```bash
cd front

# All tests (unit + e2e)
bun run test

# Unit tests only
bun run test:unit

# E2E tests only
bun run test:e2e

# Tests with coverage
bun run test:unit -- --coverage

# Storybook
bun run storybook
```

### Backend Testing

**Test Types:**
- **Unit Tests**: Native Rust testing for individual functions
- **Integration Tests**: Actix Web test utilities for API endpoints
- **Database Tests**: In-memory SurrealDB for data persistence
- **Mock Tests**: Wiremock for external service dependencies

**Running Backend Tests:**
```bash
cd back

# All tests
cargo test

# Tests with verbose output
cargo test --verbose

# Coverage report
cargo llvm-cov --html
```

### Test Requirements

- **All PRs** must have passing tests
- **New features** must include comprehensive tests
- **Bug fixes** must include regression tests
- **Test names** should clearly describe the scenario being tested
- **Tests should be reliable** and not flaky

## Automated Testing

### Continuous Integration

We use GitHub Actions for automated testing with multiple workflows:

#### 1. Pre-commit Quality Gates (`pre-commit.yml`)
Runs on every PR to ensure code quality:
- Code formatting and linting checks
- TypeScript type checking
- Commit message format validation
- File size and TODO comment checks

#### 2. Full CI Pipeline (`ci.yml`)
Comprehensive testing on `main` and `develop` branches:
- Frontend unit and E2E tests
- Backend tests with all features
- Storybook build verification
- Security audits
- Documentation link checking

#### 3. Coverage Reporting (`coverage.yml`)
Test coverage analysis and reporting:
- Coverage collection for both frontend and backend
- Coverage threshold enforcement (70% minimum)
- PR comments with coverage reports
- Codecov integration for detailed analysis

### Running CI Locally

Before pushing, simulate the CI environment:

```bash
# Frontend checks
cd front
bun run lint && bun run check && bun run test && bun run build

# Backend checks
cd back
cargo fmt --check && cargo clippy -- -D warnings && cargo test && cargo build --release
```

### Test Writing Best Practices

1. **Use descriptive test names**:
   ```typescript
   // ❌ Bad
   test('tournament test', () => { ... });

   // ✅ Good
   test('should create tournament with valid participants', () => { ... });
   ```

2. **Follow AAA pattern** (Arrange, Act, Assert):
   ```typescript
   test('should calculate winner correctly', () => {
     // Arrange
     const tournament = createTestTournament();

     // Act
     const winner = tournament.calculateWinner();

     // Assert
     expect(winner).toBe('ExpectedWinner');
   });
   ```

3. **Test behavior, not implementation**:
   ```typescript
   // ❌ Testing implementation
   expect(component.internalState.count).toBe(5);

   // ✅ Testing behavior
   expect(screen.getByText('Count: 5')).toBeVisible();
   ```

### Coverage Targets

- **Overall Project**: 70% minimum line coverage
- **New Features**: 80% coverage target
- **Critical Paths**: 90%+ coverage (authentication, data persistence)
- **Utilities/Helpers**: Near 100% coverage

## Documentation

### Types of Documentation

- **Code comments** for complex logic
- **README files** for setup and usage instructions
- **API documentation** maintained in APIdog
- **Testing documentation** in our [Testing Guide](./.github/TESTING.md)
- **Architecture documentation** for system design decisions

### Documentation Standards

- Keep documentation **up to date** with code changes
- Use **clear, concise language**
- Include **examples and code snippets**
- Consider **different audiences** (users, contributors, maintainers)
- **Test documentation examples** to ensure they work

### API Documentation

All API endpoints are documented in APIdog at: https://la-liga-de-los-muertos.apidog.io

When adding or modifying endpoints:
1. Update the APIdog documentation
2. Include request/response examples
3. Document error responses
4. Add authentication requirements
5. Include test examples for common use cases

## Getting Help

### Resources

- **Project Documentation**: [README.md](./README.md)
- **Newcomer Guide**: [./.github/NEWCOMERS.md](./.github/NEWCOMERS.md)
- **Labels Guide**: [./.github/LABELS.md](./.github/LABELS.md)
- **Testing Guide**: [./.github/TESTING.md](./.github/TESTING.md)
- **API Documentation**: https://la-liga-de-los-muertos.apidog.io
- **GitHub Issues**: https://github.com/TuentyFaiv/liga-muertos/issues
- **GitHub Discussions**: https://github.com/TuentyFaiv/liga-muertos/discussions

### Communication

- **GitHub Issues** for bug reports and feature requests
- **GitHub Discussions** for general questions and community discussions
- **Pull Request comments** for code review discussions
- **Commit messages** for explaining specific changes

### Support

If you need help:

1. **New contributor?** Start with our [Newcomer Guide](./.github/NEWCOMERS.md)
2. **Check existing documentation** first
3. **Search closed issues** for similar problems
4. **Ask in GitHub Discussions** for general questions
5. **Create a new issue** using the appropriate template
6. **Be patient and respectful** when seeking help

### Quick Help Links
- 🌟 [First-time contributors](./.github/NEWCOMERS.md)
- 🏷️ [Understanding our labels](./.github/LABELS.md)
- 🧪 [Testing guide and examples](./.github/TESTING.md)
- 🐛 [Report a bug](https://github.com/TuentyFaiv/liga-muertos/issues/new?template=bug_report.yml)
- ✨ [Request a feature](https://github.com/TuentyFaiv/liga-muertos/issues/new?template=feature_request.yml)
- 💬 [Start a discussion](https://github.com/TuentyFaiv/liga-muertos/discussions)

## Recognition

We appreciate all contributions, whether they're:

- **Code contributions** (features, fixes, improvements)
- **Documentation improvements**
- **Bug reports and feature requests**
- **Community support and discussions**
- **Testing and quality assurance**

Contributors will be recognized in our project documentation and release notes.

---

Thank you for contributing to La Liga de los Muertos! Your efforts help make this streaming creator platform better for everyone in the community. 🎮✨
