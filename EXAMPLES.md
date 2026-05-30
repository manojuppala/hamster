# Hamster VCS Examples

## Example 1: Basic Workflow

```bash
# Initialize a new repository
mkdir my-project
cd my-project
ham init

# Create some files
echo "# My Project" > README.md
echo "fn main() {}" > main.rs

# Check status
ham status

# Add files to staging area
ham add README.md main.rs

# Commit the changes
ham commit -m "Initial commit"

# View the commit history
ham log
```

## Example 2: Working with Branches

```bash
# Create and switch to a new branch
ham branch feature-login
ham checkout feature-login

# Make changes
echo "// Login functionality" >> main.rs
ham add main.rs
ham commit -m "Add login feature"

# Switch back to main
ham checkout main

# Merge the feature branch
ham merge feature-login

# Delete the feature branch
ham branch -d feature-login
```

## Example 3: Remote Repositories

```bash
# In your local repository
cd my-project

# Add a remote repository (local path for now)
ham remote add origin /path/to/remote/my-project

# Push your changes
ham push origin main

# In another location, clone the repository
cd ~/other-location
ham clone /path/to/remote/my-project
cd my-project

# Make changes
echo "More code" >> main.rs
ham add main.rs
ham commit -m "Add more code"
ham push origin main

# Back in the original repository, fetch and view changes
cd ~/my-project
ham fetch origin
ham pull origin main
```

## Example 4: Viewing Differences

```bash
# Make some changes to a file
echo "New line" >> README.md

# View differences (working directory vs staged)
ham diff

# Add to staging
ham add README.md

# Make more changes
echo "Another line" >> README.md

# View differences again
ham diff

# Commit the staged changes
ham commit -m "Update README"

# View diff against a specific commit
ham log  # Get commit hash
ham diff <commit-hash>
```

## Example 5: Using .hamsterignore

```bash
# Create a .hamsterignore file
cat > .hamsterignore << EOF
# Ignore build artifacts
target/
*.o
*.so

# Ignore IDE files
.vscode/
.idea/

# Ignore log files
*.log
EOF

# Create some files that should be ignored
mkdir target
echo "build output" > target/output.o
echo "log data" > debug.log

# Create files that should be tracked
echo "source code" > main.rs

# Check status - ignored files won't appear
ham status

# Add all files - ignored files won't be added
ham add .
ham status
ham commit -m "Add source files, ignore build artifacts"
```

## Example 6: Complete Project Workflow

```bash
# Start a new project
mkdir awesome-app
cd awesome-app
ham init

# Set up ignore patterns
cat > .hamsterignore << EOF
target/
*.log
.env
EOF

# Create initial structure
mkdir src
echo "fn main() { println!(\"Hello Hamster!\"); }" > src/main.rs
echo "# Awesome App" > README.md

# Initial commit
ham add .
ham commit -m "Initial project structure"

# Create a feature branch
ham branch feature-config
ham checkout feature-config

# Work on the feature
echo "pub struct Config {}" > src/config.rs
ham add src/config.rs
ham commit -m "Add configuration module"

# Go back to main and create another feature
ham checkout main
ham branch feature-logging
ham checkout feature-logging

# Work on logging
echo "pub fn log(msg: &str) {}" > src/logger.rs
ham add src/logger.rs
ham commit -m "Add logging module"

# Merge features back to main
ham checkout main
ham merge feature-config
ham merge feature-logging

# View the complete history
ham log

# Clean up feature branches
ham branch -d feature-config
ham branch -d feature-logging

# View final status
ham status
ham branch
```
