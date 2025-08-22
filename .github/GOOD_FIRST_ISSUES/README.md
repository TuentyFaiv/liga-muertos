# Good First Issues - Quick Setup Guide

This directory contains ready-to-use issue templates for creating newcomer-friendly issues in La Liga de los Muertos.

## 🚀 Quick Setup Instructions

### 1. Copy & Create Issues

Copy the content from each `.md` file in this directory and create GitHub issues:

1. Go to https://github.com/TuentyFaiv/liga-muertos/issues/new
2. Copy the content from one of the template files below
3. Paste it as the issue description
4. Add the suggested labels (listed at the bottom of each template)
5. Create the issue

### 2. Available Issue Templates

| Template | Description | Time | Skills |
|----------|-------------|------|--------|
| [`001-readme-troubleshooting.md`](001-readme-troubleshooting.md) | Add troubleshooting section to README | 1-2h | Writing, Markdown |
| [`002-loading-spinner.md`](002-loading-spinner.md) | Add loading spinner to auth button | 2-3h | Svelte, TailwindCSS |
| [`003-error-messages.md`](003-error-messages.md) | Improve API error handling | 3-4h | TypeScript, API handling |
| [`004-dependency-updates.md`](004-dependency-updates.md) | Update project dependencies | 2-3h | Package management |
| [`005-mobile-navigation.md`](005-mobile-navigation.md) | Improve mobile navigation | 3-4h | Responsive design |

### 3. Required Labels for Each Issue

Make sure to create these labels in your GitHub repository first:

**Essential Labels:**
- `good first issue` (#7057ff) - Perfect for newcomers
- `beginner friendly` (#b760ff) - Suitable for beginners
- `help wanted` (#159818) - Extra attention needed

**Type Labels:**
- `type: documentation` (#0075ca)
- `type: feature` (#a2eeef)
- `type: chore` (#fef2c0)

**Component Labels:**
- `component: frontend` (#ff7f00)
- `component: backend` (#8b4513)
- `component: auth` (#4c1)
- `component: api` (#b60205)

**Priority Labels:**
- `priority: medium` (#fbca04)
- `priority: high` (#ff6b35)

### 4. Creating Labels (GitHub CLI Method)

```bash
# Essential newcomer labels
gh label create "good first issue" --color "7057ff" --description "Perfect for newcomers (1-3 hours)"
gh label create "beginner friendly" --color "b760ff" --description "Suitable for beginners (3-6 hours)"
gh label create "help wanted" --color "159818" --description "Extra attention needed"

# Type labels
gh label create "type: documentation" --color "0075ca" --description "Documentation improvements"
gh label create "type: feature" --color "a2eeef" --description "New feature or enhancement request"
gh label create "type: chore" --color "fef2c0" --description "Maintenance tasks and housekeeping"

# Component labels
gh label create "component: frontend" --color "ff7f00" --description "SvelteKit frontend related"
gh label create "component: backend" --color "8b4513" --description "Rust backend related"
gh label create "component: auth" --color "4c1" --description "Authentication (Clerk/Twitch)"
gh label create "component: api" --color "b60205" --description "API related"

# Priority labels
gh label create "priority: medium" --color "fbca04" --description "Medium priority items"
gh label create "priority: high" --color "ff6b35" --description "High priority features or important bugs"

# Special labels
gh label create "dependencies" --color "0366d6" --description "Updates to dependencies"
```

## 🎯 Issue Creation Checklist

For each issue you create:

- [ ] Copy template content completely
- [ ] Add appropriate labels (listed at bottom of template)
- [ ] Set yourself as assignee (if you're the maintainer)
- [ ] Add to project board (if you have one)
- [ ] Double-check all links work correctly
- [ ] Ensure the acceptance criteria are clear
- [ ] Verify the estimated time is realistic

## 📋 Maintaining Good First Issues

### Best Practices:
1. **Keep 3-5 issues available** at all times
2. **Respond quickly** to newcomer questions (within 24-48 hours)
3. **Be encouraging** and patient with new contributors
4. **Provide clear feedback** during code review
5. **Celebrate contributions** when PRs are merged

### When Issues Are Completed:
1. Close the issue with a celebratory comment
2. Thank the contributor publicly
3. Create 1-2 new good first issues to maintain the pipeline
4. Consider increasing the difficulty slightly for returning contributors

## 🤝 Mentoring New Contributors

### For Each New Contributor:
1. **Welcome them warmly** in their first comment
2. **Assign the issue quickly** (within 24 hours)
3. **Check in proactively** if they seem stuck
4. **Review PRs promptly** (within 2-3 days)
5. **Provide constructive feedback** with specific suggestions
6. **Invite them to contribute again** after their first PR is merged

### Common Questions & Answers:
- **"I'm new to [technology]"** → Point them to learning resources
- **"I can't find the file"** → Provide specific file paths
- **"Is this still available?"** → Assign immediately if yes
- **"How do I test this?"** → Provide step-by-step testing instructions

## 🚀 Next Steps After Setup

1. **Create the first 5 issues** using these templates
2. **Announce in GitHub Discussions** that you're looking for contributors
3. **Share on social media** to attract new contributors
4. **Monitor and respond** to contributor activity
5. **Create new issues** as these get completed

## 📊 Tracking Success

Monitor these metrics:
- Number of first-time contributors
- Time to first response on issues
- PR completion rate
- Contributor retention (how many come back)
- Community growth

## 🎮 Remember

La Liga de los Muertos is for the streaming community - make sure new contributors understand they're helping build tools for streamers and their audiences. This context helps motivate contributors and creates a sense of purpose beyond just coding practice.

---

**Need help?** Create an issue with the `type: question` label or start a discussion!
