rust-version:
	rustc --version 		# rustc compiler
	cargo --version 		# cargo package manager
	rustfmt --version 		# rust formatter
	rustup --version 		# rust toolchain manager
	clippy-driver --version	# rust linter

format:
	cargo fmt

test:
	cargo test

run:
	cargo run -r

release:
	cargo build -r
	strip target/release/prepare-commit-msg

lint:
	cargo clippy

install:
	cargo build -r
	strip target/release/prepare-commit-msg
	$(eval GIT_HOOKS_PATH := $(shell git config --global core.hooksPath))
	@if [ -z "$(GIT_HOOKS_PATH)" ]; then \
		GIT_HOOKS_PATH="~/.git_hooks"; \
	fi
	mkdir -p $(GIT_HOOKS_PATH)
	cp target/release/prepare-commit-msg $(GIT_HOOKS_PATH)/prepare-commit-msg

clean:
	cargo clean
	rm -rf target
