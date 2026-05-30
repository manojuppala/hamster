# Hamster VCS - Quick Reference

## Essential Commands

| Command | Description | Example |
|---------|-------------|---------|
| `ham init` | Initialize new repository | `ham init` |
| `ham add <files>` | Add files to staging | `ham add file.txt` or `ham add .` |
| `ham commit -m "msg"` | Commit staged changes | `ham commit -m "Initial commit"` |
| `ham status` | Show working tree status | `ham status` |
| `ham log` | Show commit history | `ham log` or `ham log -n 5` |
| `ham diff [commit]` | Show changes | `ham diff` or `ham diff abc123` |

## Branch Commands

| Command | Description | Example |
|---------|-------------|---------|
| `ham branch` | List branches | `ham branch` |
| `ham branch <name>` | Create branch | `ham branch feature-x` |
| `ham branch -d <name>` | Delete branch | `ham branch -d old-feature` |
| `ham checkout <branch>` | Switch branch | `ham checkout main` |
| `ham merge <branch>` | Merge branch | `ham merge feature-x` |

## Remote Commands

| Command | Description | Example |
|---------|-------------|---------|
| `ham remote add <name> <url>` | Add remote | `ham remote add origin /path/to/repo` |
| `ham remote remove <name>` | Remove remote | `ham remote remove origin` |
| `ham remote list` | List remotes | `ham remote list -v` |
| `ham clone <url> [dir]` | Clone repository | `ham clone /path/to/repo my-repo` |
| `ham push [remote] [branch]` | Push changes | `ham push origin main` |
| `ham fetch [remote]` | Fetch changes | `ham fetch origin` |
| `ham pull [remote] [branch]` | Pull changes | `ham pull origin main` |

## File Patterns (.hamsterignore)

Create a `.hamsterignore` file in your repository root:

```
# Comments
target/
*.log
.DS_Store
node_modules/
```

## Common Workflows

### Start New Project
```bash
mkdir my-project && cd my-project
ham init
# ... create files ...
ham add .
ham commit -m "Initial commit"
```

### Feature Branch Workflow
```bash
ham branch feature-xyz
ham checkout feature-xyz
# ... make changes ...
ham add .
ham commit -m "Add feature"
ham checkout main
ham merge feature-xyz
ham branch -d feature-xyz
```

### Collaborate with Remote
```bash
# Setup
ham remote add origin /path/to/remote

# Push your work
ham add .
ham commit -m "My changes"
ham push origin main

# Get others' work
ham fetch origin
ham pull origin main
```

## Tips

- Use `ham status` frequently to see what's changed
- Commit often with descriptive messages
- Create branches for new features
- Use `.hamsterignore` to exclude build artifacts
- Check `ham log` to review history

## Getting Help

- See `README.md` for full documentation
- See `EXAMPLES.md` for detailed examples
- See `ARCHITECTURE.md` for internals
- See `BUILD.md` for build instructions
