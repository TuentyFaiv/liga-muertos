# 🌟 Welcome Newcomers to La Liga de los Muertos!

Welcome to La Liga de los Muertos! We're excited that you're interested in contributing to our streaming creator platform. This guide will help you make your first contribution with confidence.

## 🎯 What is La Liga de los Muertos?

La Liga de los Muertos is a modern web platform for managing and viewing streaming football leagues. We're building a better alternative to existing solutions, specifically designed for the streaming creator community with features like:

- 🎮 Twitch integration for seamless authentication
- ⚡ Modern SvelteKit frontend with responsive design
- 🦀 High-performance Rust backend
- 🏆 Tournament bracket management
- 📊 Real-time statistics and updates

## 🚀 Quick Start for New Contributors

### 1. Choose Your First Issue

Look for issues labeled with:
- 🌟 `good first issue` - Perfect for your first contribution (1-3 hours)
- 💜 `beginner friendly` - Good for those with some experience (3-6 hours)
- 💚 `help wanted` - Issues where we especially need help

**Where to find them:**
- [Good First Issues](https://github.com/TuentyFaiv/liga-muertos/labels/good%20first%20issue)
- [Beginner Friendly Issues](https://github.com/TuentyFaiv/liga-muertos/labels/beginner%20friendly)
- [Help Wanted Issues](https://github.com/TuentyFaiv/liga-muertos/labels/help%20wanted)

### 2. Understanding Our Tech Stack

Don't worry if you're not familiar with everything! We'll help you learn:

**Frontend (in `front/` directory):**
- SvelteKit 2 - Modern web framework
- TailwindCSS - Utility-first styling
- TypeScript - Type-safe JavaScript
- Storybook - Component development

**Backend (in `back/` directory):**
- Rust - Systems programming language
- Actix Web - Web framework
- SurrealDB Cloud - Database

**New to these technologies?** That's okay! Start with frontend issues if you know HTML/CSS/JavaScript, or documentation issues if you're completely new.

### 3. Making Your First Contribution

#### Step 1: Set Up Your Environment

```bash
# Fork the repository on GitHub, then clone your fork
git clone git@github.com:YOUR_USERNAME/liga-muertos.git
cd liga-muertos

# Set up the frontend (if working on frontend issues)
cd front
bun install
bun run dev

# Or set up the backend (if working on backend issues)
cd back
cargo build
cargo run
```

#### Step 2: Create a Branch

```bash
# Always create a new branch for your changes
git checkout -b feat/your-feature-name

# Example branch names:
# feat/add-loading-spinner
# fix/button-alignment
# docs/update-setup-guide
```

#### Step 3: Make Your Changes

- Read the issue description carefully
- Look at the "Helpful Resources" section in the issue
- Don't hesitate to ask questions in the issue comments
- Make small, focused changes

#### Step 4: Test Your Changes

```bash
# For frontend changes
cd front
bun run test
bun run lint

# For backend changes
cd back
cargo test
cargo fmt
```

#### Step 5: Submit Your Pull Request

```bash
# Commit with our format (gitmoji + prefix)
git commit -m "✨ FEAT: Add loading spinner to login button" -m "Added spinner component to improve user experience during authentication"

# Push your branch
git push origin feat/your-feature-name
```

Then create a Pull Request on GitHub using our template!

## 💡 Types of Good First Issues

### 📝 Documentation (Great for beginners!)
- Fix typos in README files
- Add code comments
- Improve setup instructions
- Translate content

**Skills needed:** Writing, attention to detail
**Time:** 30 minutes - 2 hours

### 🎨 UI/UX Improvements
- Update button styles
- Fix responsive design issues
- Add loading states
- Improve accessibility

**Skills needed:** HTML, CSS, basic JavaScript
**Time:** 1-3 hours

### 🐛 Simple Bug Fixes
- Fix broken links
- Correct form validation
- Fix minor display issues
- Update dependencies

**Skills needed:** Basic programming in the relevant language
**Time:** 1-4 hours

### 🔧 Small Features
- Add utility functions
- Create simple components
- Implement basic validations
- Add error handling

**Skills needed:** Programming knowledge, willingness to learn
**Time:** 2-6 hours

## 🤝 Getting Help

### Before You Start
1. **Read the issue carefully** - All information you need should be there
2. **Check if it's still available** - Comment "I'd like to work on this!"
3. **Ask questions early** - Better to clarify than assume

### While Working
- **Stuck on setup?** Check our [Contributing Guide](../CONTRIBUTING.md)
- **Need technical help?** Comment on the issue or start a [Discussion](https://github.com/TuentyFaiv/liga-muertos/issues)
- **Found a bug?** Create a new issue with our bug report template

### Communication Tips
- **Be patient** - Maintainers are volunteers with day jobs
- **Be specific** - "It doesn't work" is less helpful than "I get error X when doing Y"
- **Be respectful** - Follow our [Code of Conduct](../CODE_OF_CONDUCT.md)

## 🏆 What Happens After Your Contribution?

### Code Review Process
1. **Automatic checks** run (tests, linting)
2. **Maintainer review** - We'll provide feedback
3. **Address feedback** - Make requested changes
4. **Approval** - Once everything looks good
5. **Merge** - Your code becomes part of the project! 🎉

### Recognition
- Your contribution is acknowledged in release notes
- You're added as a contributor
- You get that satisfying green square on GitHub!
- You're invited to continue contributing

## 🌱 Growing as a Contributor

### After Your First Contribution
1. **Celebrate!** You're officially a contributor 🎉
2. **Look for more issues** - Try slightly more complex ones
3. **Help other newcomers** - Answer questions, share your experience
4. **Suggest improvements** - You now have insight into the project

### Path to Regular Contributor
- **3+ contributions:** Become a recognized regular contributor
- **10+ contributions:** Potential invitation to maintainer discussions
- **Significant contributions:** Possible maintainer role

## 🎮 Why Your Contribution Matters

Every contribution helps:
- **Streamers** get better tools for their communities
- **Developers** learn and grow their skills
- **Community** build something amazing together
- **You** gain experience and recognition

## 🚨 Common Newcomer Mistakes (and how to avoid them)

### ❌ Working on unassigned issues
**Solution:** Always comment and wait for assignment

### ❌ Making massive changes
**Solution:** Start small, focus on one thing at a time

### ❌ Not following commit format
**Solution:** Use our gitmoji + prefix format (see [Contributing Guide](../CONTRIBUTING.md))

### ❌ Not testing changes
**Solution:** Always run tests before submitting

### ❌ Ignoring the PR template
**Solution:** Fill out all sections of the template

## 📚 Learning Resources

### For SvelteKit (Frontend)
- [SvelteKit Tutorial](https://kit.svelte.dev/docs)
- [Svelte Interactive Tutorial](https://svelte.dev/tutorial)

### For Rust (Backend)
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)

### For Git/GitHub
- [Git Handbook](https://guides.github.com/introduction/git-handbook/)
- [GitHub Flow](https://guides.github.com/introduction/flow/)

### For Open Source
- [How to Contribute to Open Source](https://opensource.guide/how-to-contribute/)
- [First Contributions](https://firstcontributions.github.io/)

## 🤔 FAQ

**Q: I'm completely new to programming. Can I still contribute?**
A: Absolutely! Start with documentation issues or simple bug fixes. Everyone starts somewhere.

**Q: Do I need to know Rust and Svelte to contribute?**
A: Not at all! You can contribute to documentation, work on frontend-only or backend-only issues, or learn as you go.

**Q: How long should I wait for a response?**
A: Usually 24-48 hours for initial triage, up to a week for detailed reviews. We're all volunteers!

**Q: What if I can't finish an issue?**
A: That's okay! Just comment on the issue letting us know, and we'll reassign it.

**Q: Can I suggest new features?**
A: Yes! Create a feature request issue first to discuss it with the community.

## 🎉 Ready to Contribute?

1. Browse [Good First Issues](https://github.com/TuentyFaiv/liga-muertos/labels/good%20first%20issue)
2. Find one that interests you
3. Comment "I'd like to work on this!"
4. Wait for assignment
5. Start coding!

**Welcome to the La Liga de los Muertos community! We can't wait to see your contribution! 🚀**

---

*Having trouble? Reach out in [Discussions](https://github.com/TuentyFaiv/liga-muertos/issues) or tag @TuentyFaiv in your issue for help.*
