# GitHub Labels Guide

This document outlines the labeling system for La Liga de los Muertos to help maintainers organize issues and guide contributors effectively.

## 🏷️ Label Categories

### Priority Labels
| Label | Color | Description | Usage |
|-------|-------|-------------|--------|
| `priority: critical` | `#d73a49` | Critical issues that block functionality | Production bugs, security issues |
| `priority: high` | `#ff6b35` | High priority features or important bugs | Core features, major improvements |
| `priority: medium` | `#fbca04` | Medium priority items | Nice-to-have features, minor bugs |
| `priority: low` | `#0e8a16` | Low priority or future considerations | Documentation, small enhancements |

### Type Labels
| Label | Color | Description | Usage |
|-------|-------|-------------|--------|
| `type: bug` | `#d73a49` | Something isn't working correctly | Bug reports and fixes |
| `type: feature` | `#a2eeef` | New feature or enhancement request | Feature requests and implementations |
| `type: documentation` | `#0075ca` | Documentation improvements | README, guides, comments |
| `type: refactor` | `#5319e7` | Code refactoring without new features | Code cleanup and optimization |
| `type: chore` | `#fef2c0` | Maintenance tasks and housekeeping | Dependencies, build config |
| `type: question` | `#d876e3` | Questions or discussions | Support requests, clarifications |

### Component Labels
| Label | Color | Description | Usage |
|-------|-------|-------------|--------|
| `component: frontend` | `#ff7f00` | SvelteKit frontend related | UI, components, routing |
| `component: backend` | `#8b4513` | Rust backend related | API, services, database |
| `component: database` | `#006b75` | SurrealDB related | Schema, queries, migrations |
| `component: auth` | `#4c1` | Authentication (Clerk/Twitch) | Login, permissions, user management |
| `component: api` | `#b60205` | API related | Endpoints, documentation, integration |
| `component: deployment` | `#1d76db` | Deployment and infrastructure | CI/CD, hosting, configuration |
| `component: testing` | `#0e8a16` | Testing related | Unit tests, E2E, test infrastructure |

### Experience Level Labels

#### 🌟 Newcomer-Friendly Labels
| Label | Color | Description | Time Estimate | Skills Needed |
|-------|-------|-------------|---------------|---------------|
| `good first issue` | `#7057ff` | Perfect for newcomers | 1-3 hours | Basic Git, willingness to learn |
| `beginner friendly` | `#b760ff` | Suitable for beginners | 3-6 hours | Some experience with the tech stack |
| `help wanted` | `#159818` | Extra attention needed | Varies | Depends on the specific issue |

#### 🚀 Advanced Labels
| Label | Color | Description | Time Estimate | Skills Needed |
|-------|-------|-------------|---------------|---------------|
| `advanced` | `#d93f0b` | Requires deep knowledge | 1-2 days | Expert knowledge of the codebase |
| `complex` | `#b60205` | Complex architecture changes | 3+ days | System design, multiple components |

### Status Labels
| Label | Color | Description | Usage |
|-------|-------|-------------|--------|
| `status: triage` | `#ededed` | Needs initial review and categorization | New issues awaiting triage |
| `status: accepted` | `#0e8a16` | Issue accepted and ready to work on | Issues approved for development |
| `status: in progress` | `#fbca04` | Currently being worked on | Active development |
| `status: blocked` | `#d73a49` | Blocked by external dependencies | Waiting for decisions/resources |
| `status: needs review` | `#ff9500` | Ready for code review | Pull requests awaiting review |
| `status: needs testing` | `#1d76db` | Implementation complete, needs testing | QA phase |

### Special Labels
| Label | Color | Description | Usage |
|-------|-------|-------------|--------|
| `duplicate` | `#cccccc` | Duplicate issue | Mark duplicates before closing |
| `wontfix` | `#ffffff` | Not planned to be fixed | Issues that won't be addressed |
| `invalid` | `#e6e6e6` | Invalid issue or request | Malformed or off-topic issues |
| `dependencies` | `#0366d6` | Updates to dependencies | Package updates, security patches |
| `breaking change` | `#b60205` | Introduces breaking changes | Major version changes |
| `hacktoberfest` | `#ff8c69` | Suitable for Hacktoberfest | October contribution event |

## 🌟 Good First Issues Guidelines

### What Makes a Good First Issue?

1. **Clear Description**: Well-defined problem with expected outcome
2. **Limited Scope**: Can be completed in 1-3 hours
3. **Self-Contained**: Doesn't require deep architecture knowledge
4. **Good Documentation**: Links to relevant files and resources
5. **Clear Acceptance Criteria**: Specific requirements for completion

### Good First Issue Categories

#### 📝 Documentation
- Fix typos or grammar in README files
- Add code comments to existing functions
- Create or improve setup documentation
- Translate documentation to other languages

#### 🎨 UI/UX Improvements
- Update button styles or colors
- Improve responsive design for mobile
- Add loading states to components
- Fix minor accessibility issues

#### 🐛 Simple Bug Fixes
- Fix broken links in documentation
- Correct minor validation issues
- Fix simple display bugs
- Update outdated dependencies

#### 🔧 Small Features
- Add new utility functions
- Implement simple form validations
- Add basic error handling
- Create simple UI components

### Good First Issue Template

When creating a good first issue, use this template:

```markdown
## 📝 Description
Brief description of the issue and why it needs to be fixed.

## 🎯 Expected Outcome
Clear description of what should happen after the fix.

## 📋 Steps to Reproduce (for bugs)
1. Step one
2. Step two
3. Step three

## ✅ Acceptance Criteria
- [ ] Criterion 1
- [ ] Criterion 2
- [ ] Tests pass
- [ ] Documentation updated (if needed)

## 🔗 Helpful Resources
- Link to relevant files
- Documentation references
- Similar examples in the codebase

## 💡 Hints for Implementation
- Suggestion 1
- Suggestion 2
- Files to look at: `src/example.js`

## 🤝 Getting Help
- Comment on this issue if you need clarification
- Join our discussions for general questions
- Tag @TuentyFaiv for urgent blockers
```

## 🚀 Contributor Onboarding Process

### For Newcomers

1. **Choose an Issue**: Look for `good first issue` or `beginner friendly` labels
2. **Comment**: Express interest and ask questions if needed
3. **Assignment**: Maintainers will assign the issue to you
4. **Development**: Work on the issue following our [Contributing Guidelines](../CONTRIBUTING.md)
5. **Pull Request**: Submit PR using our template
6. **Review**: Participate in the code review process
7. **Celebrate**: Your contribution is merged! 🎉

### For Maintainers

1. **Triage**: Apply appropriate labels within 24-48 hours
2. **Mentorship**: Provide guidance to newcomers
3. **Review**: Prioritize reviews for newcomer PRs
4. **Recognition**: Acknowledge contributions in release notes
5. **Feedback**: Help contributors improve for future contributions

## 🏆 Recognition Program

### Contributor Levels

- **🌱 First Timer**: Merged your first PR
- **🌿 Regular**: 3+ merged PRs
- **🌳 Veteran**: 10+ merged PRs or significant contributions
- **🏅 Maintainer**: Trusted with repository access

### Recognition Methods

- Monthly contributor highlights in discussions
- Special mention in release notes
- Contributor badge in README (for regular contributors)
- Invitation to maintainer team (for veterans)

## 📊 Label Usage Statistics

Track label usage to improve the system:

- Monitor which labels are most/least used
- Identify patterns in newcomer contributions
- Adjust label system based on community feedback
- Regular label cleanup and maintenance

---

## 🤝 How to Apply Labels

### For Issues
```bash
# Add labels when creating or updating issues
gh issue edit 123 --add-label "good first issue,component: frontend,type: bug"
```

### For Pull Requests
```bash
# Add labels to PRs
gh pr edit 456 --add-label "component: backend,type: feature"
```

### Automation
Consider GitHub Actions for automatic labeling:
- Auto-label based on file paths
- Add `needs review` when PR is ready
- Add `good first issue` based on issue template

---

*This label system is designed to grow with the project. Suggest improvements by creating an issue with the `type: documentation` label!*
