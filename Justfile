FEATURES := "--all-features"
TARGETS := "--all-targets"
NO_DEPS := "--no-deps"

alias d := doc
alias do := doc-open
alias l := lint
alias ok := ci
alias t := test
alias un := udeps

default:
    @just -l

# Tests and lints.
ci: lint test

# Clean build artifacts.
clean:
    cargo clean

# Generates documentation.
doc:
    cargo doc {{ NO_DEPS }}

# Generates documentation and opens it.
doc-open:
    cargo doc {{ NO_DEPS }} --open

# Check formatting and run clippy on all targets with all features.
lint:
    cargo fmt --all -- --check
    cargo clippy {{ FEATURES }} -- -D warnings

# Format and fix clippy on all targets with all features
lintmut:
    cargo fmt --all
    cargo clippy {{ FEATURES }} --fix

# Run all tests, or just one module.
test module='':
    if [ -z '{{ module }}' ]; then \
      cargo test --doc && \
      cargo nextest r {{ FEATURES }} --no-tests pass --workspace; \
    else \
      cargo test --doc -p namespacedkey_{{ module }} && \
      cargo nextest r {{ FEATURES }} -p namespacedkey_{{ module }} --no-tests pass; \
    fi

# Runs `cargo-udeps` on all targets.
udeps:
    cargo +nightly udeps {{ TARGETS }}

# Publish all crates to crates.io.
publish:
    ./publish.sh
