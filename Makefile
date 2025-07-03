# Root Makefile for joblin workspace

# Binaries
CTL=joblinctl
SVR=joblinsvr

# Commands
CARGO=cargo

.PHONY: all build test clean run-ctl run-svr check fmt

# Build all workspace members
all: build

build:
	$(CARGO) build

# Run binaries
run-ctl:
	$(CARGO) run -p $(CTL) -- $(ARGS)

run-svr:
	$(CARGO) run -p $(SVR) -- $(ARGS)

# Run tests (for all or individual crates)
test:
	$(CARGO) test

# Format all code
fmt:
	$(CARGO) fmt

# Check for warnings and issues
check:
	$(CARGO) check

# Clean the target directory
clean:
	$(CARGO) clean
