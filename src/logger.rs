use std::env;

use env_logger::{Builder, Env};

/// Default log level used when `RUST_LOG` is not set in the environment.
const DEFAULT_LOG_LEVEL: &str = "info";

/// Resolve the effective log filter: honor a non-empty `RUST_LOG` value,
/// otherwise fall back to [`DEFAULT_LOG_LEVEL`].
fn resolve_log_filter(rust_log: Option<String>) -> String {
    match rust_log {
        Some(value) if !value.trim().is_empty() => value,
        _ => DEFAULT_LOG_LEVEL.to_string(),
    }
}

/// Initialize the global logger with a compact, timestamp-free format.
pub fn setup_logger() {
    let filter = resolve_log_filter(env::var("RUST_LOG").ok());

    Builder::from_env(Env::default().filter_or("RUST_LOG", filter))
        .format_timestamp(None)
        .format_indent(None)
        .format_target(false)
        .init()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uses_default_when_unset() {
        assert_eq!(resolve_log_filter(None), DEFAULT_LOG_LEVEL);
    }

    #[test]
    fn uses_default_when_empty_or_whitespace() {
        assert_eq!(resolve_log_filter(Some("".to_string())), DEFAULT_LOG_LEVEL);
        assert_eq!(
            resolve_log_filter(Some("   ".to_string())),
            DEFAULT_LOG_LEVEL
        );
    }

    #[test]
    fn honors_explicit_value() {
        assert_eq!(resolve_log_filter(Some("debug".to_string())), "debug");
        assert_eq!(
            resolve_log_filter(Some("rilis=trace".to_string())),
            "rilis=trace"
        );
    }
}
