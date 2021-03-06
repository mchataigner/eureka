.PHONY: build check ci clean fmt install link lint release run test

BIN_NAME = eureka
CARGO = $(shell which cargo)

build:
	@$(CARGO) build

check:
	$(CARGO) check --release

# ci: install-rustfmt lint check test
ci:
	@echo "Not implemented yet"

clean:
	rm -rf ./target

clippy:
	@$(CARGO) +nightly clippy

fmt:
	@$(CARGO) fmt

install:
	@cp ./target/release/$(BIN_NAME) /usr/local/bin/$(BIN_NAME)

link:
	@ln -sf ./target/debug/$(BIN_NAME) .

lint:
	cargo fmt --all -- --check

# TODO: In CI - verify that packaged .cargo file has reasonable size
package:
	@$(CARGO) package --allow-dirty

publish:
	@$(CARGO) publish

release:
	@$(CARGO) build --release

run:
	@RUST_BACKTRACE=1 $(CARGO) run

test:
	@$(CARGO) test -- --nocapture
