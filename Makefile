# Root Makefile for joblin workspace

# Binaries
CTL=joblinctl
SVR=joblinsvr

# Commands
CARGO=cargo

.PHONY: all build test clean run-ctl run-svr check fmt install uninstall

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

# Install joblinsvr as a systemd service
install: build
	cargo build -p $(SVR) --release
	install -m 0755 target/release/joblinsvr /usr/local/bin/joblinsvr
	install -m 0644 joblinsvr/joblinsvr.service /etc/systemd/system/joblinsvr.service
	systemctl daemon-reload
	systemctl enable --now joblinsvr.service

# Uninstall joblinsvr systemd service
uninstall:
	systemctl stop joblinsvr.service || true
	systemctl disable joblinsvr.service || true
	rm -f /usr/local/bin/joblinsvr
	rm -f /etc/systemd/system/joblinsvr.service
	systemctl daemon-reload


