//! Release version — bump only `[package].version` in `Cargo.toml`.

/// Semantic version from `Cargo.toml` (e.g. `"1.0.0"`).
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_matches_cargo_toml() {
        let manifest = include_str!("../Cargo.toml");
        let expected = manifest
            .lines()
            .find_map(|line| {
                let line = line.trim();
                line.strip_prefix("version = ")
                    .and_then(|rest| rest.strip_prefix('"'))
                    .and_then(|v| v.strip_suffix('"'))
                    .map(str::to_string)
            })
            .expect("version in Cargo.toml");
        assert_eq!(VERSION, expected);
    }
}
