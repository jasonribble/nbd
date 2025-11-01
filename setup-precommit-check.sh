#!/bin/bash
# Setup git hooks for this repository

# Create pre-commit hook
cat > .git/hooks/pre-commit << 'EOF'
#!/bin/sh
# Pre-commit hook to run cargo sqlx prepare and stage any changes

echo "Running cargo sqlx prepare..."
cargo sqlx prepare -- --all-targets --all-features

# Check if sqlx prepare generated any changes
if ! git diff --quiet .sqlx/; then
    echo "SQLx prepare generated changes, adding to commit..."
    git add .sqlx/
fi

# Run cargo fmt
echo "Running cargo fmt..."
cargo fmt --all

# Run cargo clippy
echo "Running cargo clippy..."
if ! cargo clippy; then
    echo "Clippy found issues. Please fix them before committing."
    exit 1
fi

echo "Pre-commit checks passed!"
EOF

chmod +x .git/hooks/pre-commit
echo "Git hooks installed successfully!"
echo "Run 'cargo sqlx prepare -- --all-targets --all-features' if you encounter database issues."