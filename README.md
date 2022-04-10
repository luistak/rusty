# rusty

A Rust learning playground


## Installation

1. Setup rust following the [installation guide](https://doc.rust-lang.org/book/ch01-01-installation.html)

2. Install [cargo-watch](https://crates.io/crates/cargo-watch) by running:
```bash
cargo install cargo-watch
```

3. Install rust nightly version:
```bash
rustup install nightly   
```

## Running

1. Build every package in this workspace:
```bash
cargo build
```

2. Running a specific package `cargo run --p PACKAGE_NAME`:
```bash
cargo run -p server

# Or
cargo watch -x 'run -p server'
```