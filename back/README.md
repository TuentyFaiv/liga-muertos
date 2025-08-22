# La Liga de los Muertos Backend

The backend service for Liga Muertos, built with Rust for high performance and reliability.

## Technology Stack

- **Language**: [Rust](https://www.rust-lang.org/)
- **Web Framework**: [Actix Web](https://actix.rs/) - A powerful, pragmatic, and extremely fast web framework for Rust
- **Database**: [SurrealDB](https://surrealdb.com/) - A scalable, distributed database system
- **API Style**: RESTful API with JSON payloads
- **Authentication**: [Clerk](https://clerk.com/) - Complete authentication and user management platform with Twitch integration

## Project Structure

```
back/
├── src/
│   ├── entities/        # SurrealDB data models and schemas
│   ├── repositories/    # Database access layer
│   ├── services/        # Business logic layer
│   ├── routes/          # HTTP request handlers (controllers)
│   ├── middleware/      # Request/response middleware
│   ├── utils/           # Utility functions and helpers
│   └── main.rs          # Application entry point
├── target/              # Compiled artifacts (generated)
├── .env.example         # Environment variables template
├── .gitignore           # Git ignore patterns
├── Cargo.lock           # Dependency lock file
├── Cargo.toml           # Project dependencies and metadata
└── README.md            # This file
```

## Architecture Overview

The backend follows a layered architecture pattern for maintainability and separation of concerns:

### 🗃️ **Entities**
SurrealDB data models and schema definitions
- Tournament structures
- User profiles
- Match records
- League configurations

### 📚 **Repositories**
Database access layer with SurrealDB integration
- CRUD operations
- Query builders
- Connection management
- Data validation

### ⚙️ **Services**
Business logic layer
- Tournament management
- User authentication
- Match processing
- Real-time updates

### 🌐 **Routes (Controllers)**
HTTP request handlers
- API endpoints
- Request validation
- Response formatting
- Error handling

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- [SurrealDB Cloud](https://surrealdb.com/cloud) account

### Installation

1. Clone the repository (if you haven't already):
   ```bash
   git clone git@github.com:TuentyFaiv/liga-muertos.git
   cd liga-muertos/back
   ```

2. Set up environment variables:
   ```bash
   cp .env.example .env
   # Edit .env with your configuration
   ```

3. Build the project:
   ```bash
   cargo build
   ```

### Development

Run the development server:
```bash
cargo run
```

This will start the backend server at http://localhost:4000 (default port).

The server will automatically connect to your SurrealDB Cloud instance and initialize the required database schemas.

### Environment Variables

Create a `.env` file in the `back` directory with the following variables:

```
PORT=4000
SURREAL_URL=your_surrealdb_cloud_endpoint
SURREAL_NAMESPACE=your_namespace
SURREAL_DATABASE=your_database
SURREAL_USER=your_username
SURREAL_PASS=your_password
CLERK_SECRET_KEY=your_clerk_secret_key
CLERK_PUBLISHABLE_KEY=your_clerk_publishable_key
```

**Note:** When deployed to Railway, the `PORT` environment variable is automatically provided. You only need to set it for local development.

### Authentication Setup

This project uses [Clerk](https://clerk.com/) for authentication and user management, with special support for Twitch integration for streaming creators.

1. Create a Clerk account at [clerk.com](https://clerk.com/)
2. Set up a new application in your Clerk dashboard
3. Configure Twitch as a social connection following the [Clerk Twitch integration guide](https://clerk.com/docs/authentication/social-connections/twitch)
4. Copy your secret and publishable keys to the `.env` file

### Database Setup

This project uses [SurrealDB Cloud](https://surrealdb.com/docs/cloud) for managed database hosting.

1. Sign up for SurrealDB Cloud at [surrealdb.com](https://surrealdb.com/)
2. Create a new database instance
3. Note down your connection details:
   - Endpoint URL
   - Namespace
   - Database name
   - Username and password
4. Update your `.env` file with the connection details

For local development, you can optionally run SurrealDB locally:
```bash
docker run --rm -p 8000:8000 surrealdb/surrealdb:latest start --user root --pass root memory
```

## API Documentation

The complete API documentation is maintained using [APIdog](https://apidog.com/), which provides interactive documentation, testing capabilities, and collaboration features.

### Accessing the Documentation

The API documentation is available on APIdog at: https://la-liga-de-los-muertos.apidog.io

### API Endpoints

All endpoints, request/response schemas, authentication flows, and examples are documented in APIdog. This includes:

### Core API Modules
- **Authentication** - Clerk integration with Twitch support
- **Tournaments** - Create, manage, and track tournament progress
- **Users** - User profiles and preferences
- **Matches** - Match scheduling, results, and statistics
- **Leagues** - League management and standings
- **Real-time** - WebSocket endpoints for live updates

For the most up-to-date API reference, please refer to the APIdog documentation.

## Testing

Run the test suite:

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run tests with coverage
cargo llvm-cov --html
```

### Test Structure
- **Unit tests** - Located alongside source code in each module
- **Integration tests** - HTTP endpoint testing with mock services
- **Repository tests** - Database interaction testing with in-memory SurrealDB

## Building for Production

```bash
cargo build --release
```

The compiled binary will be located at `target/release/liga-muertos-back`.

### Production Environment Variables
Ensure all required environment variables are set:
- SurrealDB Cloud connection details
- Clerk authentication keys
- `PORT` (automatically provided by Railway)
- Logging levels (`RUST_LOG`)

## Deployment

### Railway Deployment

This backend is designed to be deployed on [Railway](https://railway.com/), a modern hosting platform that provides easy Rust deployments.

#### Setup Steps

1. **Create Railway Account**: Sign up at [railway.com](https://railway.com/)

2. **Connect Repository**:
   - Connect your GitHub repository to Railway
   - Select the backend directory (`back/`) as the root

3. **Configure Environment Variables**:
   Set the following variables in Railway dashboard:
   ```
   SURREAL_URL=your_surrealdb_cloud_endpoint
   SURREAL_NAMESPACE=your_namespace
   SURREAL_DATABASE=your_database
   SURREAL_USER=your_username
   SURREAL_PASS=your_password
   CLERK_SECRET_KEY=your_clerk_secret_key
   CLERK_PUBLISHABLE_KEY=your_clerk_publishable_key
   RUST_LOG=info
   ```

4. **Deploy**: Railway will automatically detect the Rust project and deploy

#### Railway Benefits
- **Automatic builds** from Git commits
- **Built-in SSL/TLS** certificates
- **Custom domains** support
- **Environment variable** management
- **Monitoring and logs**
- **Automatic scaling**

#### Railway Configuration
Railway automatically:
- Sets the `PORT` environment variable
- Binds to `0.0.0.0` (all interfaces)
- Provides HTTPS endpoints
- Handles container orchestration

No additional configuration files are needed for Railway deployment.

## Contributing

We welcome contributions to the backend! Please read our [Contributing Guidelines](../CONTRIBUTING.md) for detailed information on:

- Development workflow and Git Flow
- Commit message conventions with gitmoji
- Pull request process and requirements
- Code style standards for Rust/Actix
- Testing guidelines for backend services
- API documentation practices with APIdog

Quick links:
- 📋 [Full Contributing Guide](../CONTRIBUTING.md)
- 🐛 [Report a Backend Bug](https://github.com/TuentyFaiv/liga-muertos/issues/new?template=bug_report.yml)
- ✨ [Request a Backend Feature](https://github.com/TuentyFaiv/liga-muertos/issues/new?template=feature_request.yml)
- 🔗 [Backend Development Setup](../CONTRIBUTING.md#backend-setup)
