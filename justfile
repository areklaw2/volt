export DATABASE_URL := "postgres://postgres:pa55word@localhost:5432/volt"

set working-directory := "server"

# Run the API
serve:
  cargo run

# Watch + rerun on change (requires bacon: cargo install bacon)
watch:
  bacon run-long -- --bin volt

# Run tests (cargo nextest if installed, else: cargo test)
test:
  cargo nextest run --all-features

# Lint with clippy (warnings = errors)
lint:
  cargo clippy --all-targets --all-features -- -D warnings

# Format
fmt:
  cargo fmt

# Start the local dev postgres container
db-up:
  docker run -e POSTGRES_USER=postgres -e POSTGRES_PASSWORD=pa55word -e POSTGRES_DB=volt -p 5432:5432 -d --name volt_test postgres

# Stop the local dev postgres container
db-down:
  docker stop volt_test

# Remove the local dev postgres container entirely
db-rm:
  docker rm -f volt_test

# Create the local database (uses DATABASE_URL above)
create-db:
  sqlx database create

# New migration file, e.g. just create-migration add_core_tables
create-migration name:
  sqlx migrate add --timestamp {{name}}

# Apply pending migrations
run-migration:
  sqlx migrate run

# Drop, recreate, and migrate the database
db-reset:
  sqlx database drop -y && sqlx database create && sqlx migrate run

# Update sqlx offline query metadata (.sqlx/) — commit the result
prepare:
  cargo sqlx prepare -- --tests

# Point git at the tracked .githooks (run once per clone)
setup-hooks:
  git config core.hooksPath .githooks
