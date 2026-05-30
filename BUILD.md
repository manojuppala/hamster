# Building Hamster VCS

## Prerequisites

You need Rust installed on your system. If you don't have it:

### Install Rust

**macOS/Linux:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**Windows:**
Download and run [rustup-init.exe](https://rustup.rs/)

## Building

```bash
cargo build --release
```

The compiled binary will be at: `target/release/ham`

## Installing

To install system-wide:

```bash
cargo install --path .
```

This will install the `ham` binary to `~/.cargo/bin/` (make sure it's in your PATH).

## Running Without Installing

```bash
cargo run -- init
cargo run -- add file.txt
cargo run -- commit -m "message"
```

Or use the debug build directly:

```bash
./target/debug/ham init
./target/debug/ham status
```

## Testing

Create a test directory and try it out:

```bash
mkdir test_repo
cd test_repo
ham init
echo "Hello Hamster" > README.txt
ham add README.txt
ham commit -m "Initial commit"
ham log
ham status
```
