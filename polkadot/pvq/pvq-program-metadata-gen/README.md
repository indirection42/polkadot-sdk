# PVQ Program Metadata Generator

A command-line tool for generating metadata for PVQ programs.

This tool parses your PVQ program source (`src/main.rs`), finds the `#[program]` module,
collects functions annotated with `#[program::entrypoint]` and `#[program::extension_fn(...)]`,
then compiles and runs a temporary crate that writes metadata artifacts to disk.

## Installation

You can install the tool globally using Cargo:

```bash
cargo install --path /path/to/pvq-program-metadata-gen
```

## Usage

The basic usage is as follows:

```bash
pvq-program-metadata-gen --crate-path /path/to/your/crate --output-dir /path/to/output/dir
```

### Arguments

- `--crate-path, -c`: Path to the crate directory containing a PVQ program
- `--output-dir, -o`: Output directory for the metadata files (typically from a `METADATA_OUTPUT_DIR` env var in `build.rs`)
- `--manifest-path, -m`: Optional `Cargo.toml` to use for the temporary generator crate. Use this if your program module depends on crates that are not in the default minimal manifest.
- `--target`: Optional target triple for `cargo run --target ...`

### Important notes

- The tool expects your PVQ program source at `src/main.rs` (relative to `--crate-path`).
- The output filenames are based on `CARGO_PKG_NAME` (e.g. `my-program-metadata.json`). When run from a Cargo `build.rs`, Cargo provides this variable automatically. If you run the tool manually, set it yourself (for example: `CARGO_PKG_NAME=my-program pvq-program-metadata-gen ...`).

## Integration with Build Scripts

You can integrate this tool into your crate's `build.rs` file:

```rust
use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // Tell Cargo to rerun this build script if the source file changes
    let current_dir = env::current_dir().expect("Failed to get current directory");
    // Determine the output directory for the metadata
    let output_dir = PathBuf::from(env::var("METADATA_OUTPUT_DIR").expect("METADATA_OUTPUT_DIR is not set"));

    // Build and run the command
    let status = Command::new("pvq-program-metadata-gen")
        .arg("--crate-path")
        .arg(&current_dir)
        .arg("--output-dir")
        .arg(&output_dir)
        .env("RUST_LOG", "info")
        .status()
        .expect("Failed to execute pvq-program-metadata-gen");

    if !status.success() {
        panic!("Failed to generate program metadata");
    }
}

```

## How It Works

The tool:

1. Reads the source code of your PVQ program
2. Generates metadata generation code
3. Creates a temporary crate that stores the metadata generation code
4. Compiles and runs the temporary crate (optionally with the same active feature flags)

The metadata includes function names, parameter types, and return types, so downstream tooling (for example UI code) can understand the PVQ program’s surface.
