# rusty

A Rust learning playground


## Installation

1. Setup rust following the [installation guide](https://doc.rust-lang.org/book/ch01-01-installation.html)

2. Install [cargo-expand](https://crates.io/crates/cargo-expand) running
```bash
cargo install cargo-expand
```

3. Install rust nightly version
```bash
rustup install nightly   
```

## Running

1. Build every package in this workspace
```bash
cargo build
```

2. Running a specific bin package `cargo run --bin PACKAGE_NAME` example:
```bash
cargo run -p hello-world
```
