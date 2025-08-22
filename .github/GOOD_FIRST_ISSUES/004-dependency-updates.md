# Update Dependencies to Latest Versions

## 📝 Description
Our project dependencies can become outdated over time, potentially introducing security vulnerabilities or missing out on performance improvements and bug fixes. We need to update both frontend and backend dependencies to their latest stable versions while ensuring compatibility.

## 🎯 Expected Outcome
All dependencies in both `front/package.json` and `back/Cargo.toml` should be updated to their latest stable versions, with all tests passing and no breaking changes introduced.

## ✅ Acceptance Criteria
- [ ] Update frontend dependencies in `front/package.json`
- [ ] Update backend dependencies in `back/Cargo.toml`
- [ ] Run tests to ensure no breaking changes
- [ ] Update lock files (`front/bun.lock` and `back/Cargo.lock`)
- [ ] Check for any deprecated packages and suggest replacements if needed
- [ ] Document any major version changes that might affect development
- [ ] Ensure the application still builds and runs correctly

## 🔗 Helpful Resources
- Frontend package.json: [`front/package.json`](../../front/package.json)
- Backend Cargo.toml: [`back/Cargo.toml`](../../back/Cargo.toml)
- [npm-check-updates](https://www.npmjs.com/package/npm-check-updates) for checking outdated packages
- [cargo-outdated](https://github.com/kbknapp/cargo-outdated) for Rust dependencies
- [Bun documentation](https://bun.sh/docs) for package management

## 💡 Hints for Implementation

### Frontend Dependencies Update
```bash
cd front

# Check for outdated packages
bun outdated

# Update all dependencies (be careful with major version changes)
bun update

# Or update specific packages
bun add package-name@latest

# Run tests to ensure everything works
bun run test
bun run lint
bun run build
```

### Backend Dependencies Update
```bash
cd back

# Install cargo-outdated if you don't have it
cargo install cargo-outdated

# Check for outdated dependencies
cargo outdated

# Update dependencies in Cargo.toml
# Edit Cargo.toml manually or use cargo-edit
cargo add package-name@latest

# Update Cargo.lock
cargo update

# Run tests
cargo test
cargo build
```

## ⚠️ Important Considerations

### Major Version Changes
- Review changelog for breaking changes
- Test thoroughly if updating major versions
- Consider updating one major dependency at a time
- Document any API changes in the PR description

### Security Updates
- Prioritize security-related updates
- Check for known vulnerabilities with `bun audit` or `cargo audit`
- Pay special attention to authentication and web framework updates

### Development Dependencies
- Update dev dependencies like testing frameworks, linters, and build tools
- Ensure Storybook, Playwright, and other dev tools still work correctly

## 🧪 Testing Checklist

### Frontend Testing
- [ ] `bun install` runs without errors
- [ ] `bun run dev` starts development server
- [ ] `bun run build` creates production build successfully
- [ ] `bun run test:unit` passes all unit tests
- [ ] `bun run test:e2e` passes all end-to-end tests
- [ ] `bun run lint` shows no new linting errors
- [ ] `bun run storybook` starts correctly

### Backend Testing
- [ ] `cargo build` compiles without errors
- [ ] `cargo test` passes all tests
- [ ] `cargo run` starts the server successfully
- [ ] No new compiler warnings introduced
- [ ] API endpoints still work correctly

## 📋 What to Include in Your PR

### Documentation
- List all packages that were updated with old → new versions
- Note any breaking changes or important updates
- Mention if any packages were deprecated/replaced
- Include any new environment variables or configuration needed

### Example PR Description Format
```markdown
## Updated Dependencies

### Frontend (package.json)
- @sveltejs/kit: 2.20.0 → 2.22.0
- tailwindcss: 3.4.0 → 4.0.0 ⚠️ Major update
- typescript: 5.0.0 → 5.3.0

### Backend (Cargo.toml)
- actix-web: 4.4.0 → 4.4.1
- serde: 1.0.190 → 1.0.193

### Breaking Changes
- TailwindCSS v4 requires configuration updates (see migration guide)

### Testing
- ✅ All tests pass
- ✅ Applications build and run successfully
- ✅ No new security vulnerabilities
```

## 🔍 Additional Tasks (Optional)
- Run security audit and fix any vulnerabilities
- Remove any unused dependencies
- Check if any dev dependencies can be moved to regular dependencies or vice versa
- Look for opportunities to consolidate similar packages

## 🤝 Getting Help
- Comment on this issue if you encounter build errors after updates
- Ask about specific dependencies if you're unsure about major version updates
- Request guidance on handling breaking changes
- Join our [discussions](https://github.com/TuentyFaiv/liga-muertos/issues) for general questions
- Tag @TuentyFaiv if you need help resolving conflicts or complex dependency issues

---

**Labels to apply:** `good first issue`, `type: chore`, `dependencies`, `priority: medium`, `help wanted`
**Estimated time:** 2-3 hours
**Skills needed:** Package management, basic testing, attention to detail
