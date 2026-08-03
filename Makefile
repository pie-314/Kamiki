# eBPF programs must be built on Linux (clang + libbpf required).
# Rust workspace builds on any platform for development, but can only
# run the collector on Linux (eBPF).

CARGO      ?= $(shell which cargo || echo cargo)
SUDO_CARGO := sudo env PATH="$$PATH" RUSTUP_TOOLCHAIN=stable $(CARGO)
INTERFACE  ?= $(shell ip route show default 2>/dev/null | awk '/default/ {print $$5}' | head -n1 || echo enp0s3)

.PHONY: all ebpf rust build test clean fmt clippy run-cli run-tui run-gui run-web setup



setup:
	@chmod +x .githooks/pre-commit .githooks/pre-push
	@chmod +x scripts/lint.sh scripts/test.sh
	@git config core.hooksPath .githooks
	@echo "Git hooks installed (.githooks/). Run 'make build' to verify."


all: ebpf rust

# eBPF (Linux only)

ebpf:
	$(MAKE) -C kamiki-ebpf

ebpf-clean:
	$(MAKE) -C kamiki-ebpf clean

# vmlinux.h for the current running kernel
vmlinux:
	bpftool btf dump file /sys/kernel/btf/vmlinux format c \
		> kamiki-ebpf/include/vmlinux.h
	@echo "vmlinux.h regenerated for kernel $$(uname -r)"


rust:
	$(CARGO) build --workspace

build: rust

release:
	$(CARGO) build --release --workspace

test:
	$(CARGO) test --workspace

fmt:
	$(CARGO) fmt --all

clippy:
	$(CARGO) clippy --workspace -- -D warnings


# Requires: sudo (eBPF needs CAP_BPF / CAP_NET_ADMIN), Linux, built eBPF object
run-cli:
	$(SUDO_CARGO) run -p kamiki-cli -- --interface $(INTERFACE)

run-tui:
	$(SUDO_CARGO) run -p kamiki-tui -- --interface $(INTERFACE)

run-gui:
	$(CARGO) run -p kamiki-gui

run-web:
	cd kamiki-web && dx build
	$(SUDO_CARGO) run -p kamiki-server -- --interface $(INTERFACE)



clean: ebpf-clean
	$(CARGO) clean
