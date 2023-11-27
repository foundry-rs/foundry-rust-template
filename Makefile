#!/usr/bin/make -f

# Default target is build
default: build

# Define variables
CARGO=cargo
CONTRACTS_PATH=./contracts
BINDINGS_FOLDER=bindings
BINDINGS_OUT_PATH=$(CONTRACTS_PATH)/out/$(BINDINGS_FOLDER)

# Target for generating bindings
bindings:
	# Remove old bindings
	rm -rf $(BINDINGS_FOLDER)
	rm -rf $(BINDINGS_OUT_PATH)
	
	# Generate new bindings
	@forge bind --root $(CONTRACTS_PATH) --crate-name $(BINDINGS_FOLDER)
	
	# Move bindings to the correct location
	@mv -f $(BINDINGS_OUT_PATH) .

# Target for building the project
build: bindings
	@$(CARGO) build
	
# Target for building the project in release mode
build-release: bindings
	@$(CARGO) build --release

# Target for cleaning the project
clean:
	forge clean --root $(CONTRACTS_PATH)
	@$(CARGO) clean

# Target for formatting the code
fmt:
	forge fmt --check --root $(CONTRACTS_PATH)
	@$(CARGO) fmt


# Declare phony targets
.PHONY: build build-release clean fmt bindings