/// Builds a `Cargo.toml` for the temporary metadata-generator crate.
///
/// The generated crate is executed via `cargo run` to write metadata artifacts.
/// Optionally, a `[features]` table can be embedded so the temp crate can be
/// built with the same feature flags as the original program crate.
pub fn create_manifest(features: Option<&toml::Table>) -> String {
    // Create a basic Cargo.toml for the temp crate
    format!(
        r#"[package]
name = "metadata_gen"
version = "0.1.0"
edition = "2021"

[dependencies]
scale-info = {{ version = "2.0.0", features = ["derive","serde"] }}
parity-scale-codec = {{ version = "3.0.0", features = ["derive"] }}
serde = {{ version = "1", features = ["derive" ] }}
serde_json = "1"
cfg-if = "1.0"
{0}
"#,
        features.map_or_else(String::new, |features| format!(
            "\n[features]\n{}",
            toml::to_string(&features).expect("Should be checked in parsing")
        ))
    )
}
