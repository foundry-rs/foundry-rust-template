#!/usr/bin/make -f

# Default target is build
default: build

# Define variables
CARGO=cargo
CRATES_FOLDER=crates
CONTRACTS_PATH=./contracts
BINDINGS_FOLDER=bindings

# Target for generating bindings
bindings:
# Generate new bindings
	@forge bind --bindings-path ./crates/bindings --crate-name $(BINDINGS_FOLDER) --optimize true --alloy --alloy-version v0.9.2 --overwrite

# Target for building the project
build: bindings
	@$(CARGO) build

# Target for building the project in release mode
build-release: bindings
	@$(CARGO) build --release

# Target for cleaning the project
clean:
	@forge clean --root $(CONTRACTS_PATH)
	@$(CARGO) clean

# Target for formatting the code
fmt:
	@forge fmt --check --root $(CONTRACTS_PATH)
	@$(CARGO) fmt

# Target for running tests
test:
	@forge test
	@$(CARGO) test

# Target for installing forge dependencies
setup:
	@forge install


# Declare phony targets
.PHONY: build build-release clean fmt bindings
