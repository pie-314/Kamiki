# Contributing to Kamiki



---

## First-time setup

```bash
git clone <repo>
cd Kamiki
make setup        # installs git hooks, marks scripts executable
```

That's it. `make setup` wires up `.githooks/` as your local hook directory. You do not need to run it again unless you reclone.

Verify hooks are active:
```bash
git config core.hooksPath   # should print: .githooks
```

---

## Environment

### macOS (all contributors)

Rust builds work fine on macOS. eBPF compilation does not - you need Linux for that.

```bash
brew install rustup lima
rustup toolchain install stable
```

### Linux / Lima VM (required for eBPF work)

Lima VM is how we run Linux on macOS for eBPF development:

```bash
limactl start --name=kamiki template://ubuntu-lts
limactl shell kamiki
```

Inside the VM, install build dependencies:

```bash
sudo apt install -y clang llvm libbpf-dev linux-tools-$(uname -r)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

The project root is mounted into the VM automatically via Lima's default mounts. Make sure it's writable - if `make vmlinux` fails with "Read-only file system", edit `~/.lima/kamiki/lima.yaml` and set `writable: true` on the mount entry, then `limactl stop kamiki && limactl start kamiki`.

---

## Build reference

```bash
make setup        # first time only - install git hooks

make ebpf         # compile eBPF C programs → kamiki-ebpf/out/ (Linux only)
make vmlinux      # regenerate vmlinux.h for current kernel (Linux only)
make build        # cargo build --workspace
make release      # cargo build --release --workspace

make fmt          # auto-format (run this before committing)
make clippy       # lint (hooks run this automatically)
make test         # cargo test --workspace

make run-tui      # sudo run the TUI (Linux + built eBPF object required)
make run-cli      # sudo run the CLI
make run-gui      # run the GUI (no sudo needed, no eBPF)

make clean        # wipe Rust target/ + eBPF build artifacts
```

### Full dev cycle on Linux

```bash
make vmlinux      # once after kernel update
make ebpf         # after editing any .bpf.c file
make build        # after editing any Rust file
make run-tui INTERFACE=eth0
```

### eBPF-only iteration (contributor workflow)

```bash
cd kamiki-ebpf
make              # compiles xdp_prober.bpf.c → out/xdp_prober.bpf.o
# attach manually if you want to test raw:
# sudo ip link set dev eth0 xdp obj out/xdp_prober.bpf.o sec xdp
# sudo ip link set dev eth0 xdp off
```

### Why `sudo` for run targets

XDP requires `CAP_NET_ADMIN` and `CAP_BPF`. The Makefile uses `sudo env PATH=... RUSTUP_TOOLCHAIN=stable cargo` because:
- `sudo` resets PATH, losing `~/.cargo/bin`
- rustup's cargo shim needs `RUSTUP_TOOLCHAIN` to pick a toolchain when run as root

---

## Before you push

The git hooks handle this automatically, but here's what they run:

**pre-commit** (`scripts/lint.sh`):
1. `cargo fmt --all -- --check` - fails if any file needs formatting
2. `cargo clippy --workspace -- -D warnings` - fails on any warning

**pre-push** (`scripts/lint.sh` + `scripts/test.sh`):
1. Everything from pre-commit
2. `cargo test --workspace`

If pre-commit fires and fmt fails:
```bash
make fmt          # fix formatting
git add -u        # re-stage the fixed files
git commit        # try again
```

If you need to skip hooks in an emergency (don't make a habit of it):
```bash
git commit --no-verify
git push --no-verify
```

---

## Struct sync between C and Rust

`struct pkt_event` in `kamiki-ebpf/src/xdp_prober.bpf.c` and `RawPktEvent` in `kamiki-core/src/event.rs` must stay byte-for-byte identical. A compile-time assert guards the size:

```rust
const _SIZE_CHECK: () = assert!(std::mem::size_of::<RawPktEvent>() == 20);
```

If you add a field to the C struct, you must update `RawPktEvent` in the same PR, and update the assert. Never add fields to the C struct without the corresponding Rust change - you will get silently wrong data, not a crash.

C struct padding rules: if you add a `__u8` before a `__u32`, C inserts 3 bytes of padding. Mirror that explicitly in Rust with `_pad: [u8; 3]`.

---

## Byte order

eBPF reads packet headers in network byte order (big-endian). The XDP program converts to host order with `bpf_ntohl`/`bpf_ntohs` before writing to the ring buffer. Rust userspace reads values already in host order - do not call `u32::from_be()` or `u16::from_be()` on values that came through the ring buffer.

---

## Branching

- `main` - must always build cleanly on macOS (`cargo build --workspace`) and pass tests
- Work on feature branches: `feat/process-correlation`, `fix/flow-eviction`, etc.
- PRs go into `main`. One review from any other contributor is enough to merge.
- Don't commit directly to `main` unless it's a one-liner fix.

## Commit messages

```
<area>: short description in imperative mood

Longer explanation if needed. Why, not what.
```

Examples:
```
core: add flow eviction for idle connections
ebpf: fix NULL key in bpf_map_lookup_elem
tui: add byte formatter with KB/MB/GB units
```

Keep the first line under 72 characters. If you're writing a novel in the commit message, it probably belongs in a comment in the code or in this file.

---

## Things that will get your PR bounced

- Clippy warnings (hooks catch this, but still)
- Unformatted code (hooks catch this too)
- C struct changes without matching Rust changes
- `todo!()` or `unimplemented!()` in code paths that can actually be hit at runtime - use `return Err(...)` instead, panics corrupt the TUI
- `println!` in library code - use `log::info!` / `log::debug!` so the caller controls output
- `unwrap()` on anything that can realistically fail at runtime

`unwrap()` is fine in tests and in const contexts. It's not fine in the collector loop or UI tick path.
