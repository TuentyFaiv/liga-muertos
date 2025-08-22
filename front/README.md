# La Liga de los Muertos Frontend

Modern, responsive web interface for the La Liga de los Muertos platform built with SvelteKit.

## Technologies

- [SvelteKit 2](https://kit.svelte.dev/): A framework for building web applications of all sizes
- [Svelte 5](https://svelte.dev/): Component framework with reactivity built-in
- [TailwindCSS](https://tailwindcss.com/): Utility-first CSS framework
- [shadcn-svelte](https://shadcn-svelte.com/): Copy & paste components built with Radix UI and Tailwind CSS
- [Storybook](https://storybook.js.org/): Tool for UI component development and testing
- [Playwright](https://playwright.dev/): End-to-end testing
- [Vitest](https://vitest.dev/): Unit testing framework
- [Paraglide](https://inlang.com/): Internationalization library

## Project Structure

```
front/
├── .storybook/           # Storybook configuration
├── e2e/                  # End-to-end tests
├── locales/              # Translation files
├── project.inlang/       # i18n project configuration
├── src/
│   ├── assets/           # Static assets and resources
│   ├── logic/            # Business logic and utilities
│   ├── routes/           # SvelteKit routes and pages
│   ├── stories/          # Storybook stories
│   ├── ui/               # UI components and shared elements
│   │   └── sharing/      # Shared UI components
│   ├── app.d.ts          # App-wide TypeScript definitions
│   ├── app.html          # HTML template
│   ├── hooks.server.ts   # Server-side hooks
│   └── hooks.ts          # Universal hooks (client and server)
├── static/               # Static public assets
├── biome.json           # Biome configuration
├── components.json      # shadcn-svelte components configuration
├── package.json         # Dependencies and scripts
├── playwright.config.ts # Playwright E2E configuration
├── svelte.config.js     # SvelteKit configuration
├── tsconfig.json        # TypeScript configuration
├── vite.config.ts       # Vite configuration
└── vitest-setup-client.ts # Vitest client setup
```

## Getting Started

### Prerequisites

- [Node.js](https://nodejs.org/) (v22 or newer)
- [Bun](https://bun.sh/) package manager

### Installation

1. Clone the repository (if not already done):
   ```bash
   git clone git@github.com:TuentyFaiv/liga-muertos.git
   cd liga-muertos/front
   ```

2. Install dependencies:
   ```bash
   bun install
   ```

3. Setup environment variables:
   ```bash
   cp .env.example .env
   ```

   Edit `.env` with your specific configuration values.

### Development

Run the development server:
```bash
bun run dev
```

The application will be available at `http://localhost:5173` by default.

### Storybook

Develop and test components in isolation:
```bash
bun run storybook
```

Storybook will be available at `http://localhost:6006`.

### Linting

```bash
# Check linting issues
bun run lint

# Fix linting issues
bun run lint:fix
```

### Testing

```bash
# Run end-to-end tests
bun run test:e2e

# Run unit tests
bun run test:unit

# Run all tests
bun run test
```

## Internationalization

This project uses Paraglide for internationalization. Translation files are located in the `locales` directory.

### Adding a new language

1. Create a new file in the `locales` directory (e.g., `locales/es.json` for Spanish)
2. Add translations for all keys
3. Update the language selector in the application

## Building for Production

```bash
bun run build
```

The built application will be in the `.svelte-kit/build` directory.

## Preview Production Build

```bash
bun run preview
```

## Deployment

The application is configured to deploy on Vercel using the SvelteKit adapter.

```bash
bun run build
```

## Contributing

We welcome contributions to the frontend! Please read our [Contributing Guidelines](../CONTRIBUTING.md) for detailed information on:

- Development workflow and Git Flow
- Commit message conventions with gitmoji
- Pull request process and requirements
- Code style standards for SvelteKit
- Testing guidelines for frontend components
- Storybook development practices

Quick links:
- 📋 [Full Contributing Guide](../CONTRIBUTING.md)
- 🐛 [Report a Frontend Bug](https://github.com/TuentyFaiv/liga-muertos/issues/new?template=bug_report.yml)
- ✨ [Request a Frontend Feature](https://github.com/TuentyFaiv/liga-muertos/issues/new?template=feature_request.yml)
- 🔗 [Frontend Development Setup](../CONTRIBUTING.md#frontend-setup)

## License

This project is licensed under the MIT License - see the [LICENSE](../LICENSE) file for details.

The MIT License allows you to use, modify, and distribute this software freely, including for commercial purposes, as long as you include the original copyright notice.
