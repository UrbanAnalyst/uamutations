all: buildall run test fmt lint

# Note that 'make help' needs the '## <stuff>' to parse commands
build: ## main build fn
	cargo build --lib

buildall:
	cargo build

run: ## run binary
	cargo run

test: ## run tests
	cargo test

fmt: ## format code but don't lint
	cargo fmt

coverage: ## coverage via tarpaulin
	# cargo llvm-coverage
	cargo tarpaulin

lint: ## lint code but don't format
	cargo clippy

doc: ## Render docs
	cargo doc

open: ## Open documentation in local browser
	cargo doc --open

kill: ## kill YouCompleteMe rust-analyzer
	@pkill -f bin/rust-analyzer || true

help: ## Show this help
		@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'
