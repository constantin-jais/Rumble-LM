//! presto-core — the shared client/protocol core for Presto-Matic.
//!
//! Compiled to native (via UniFFI) and to wasm (web). Phase 0 ships only this
//! scaffold; the realtime session protocol, shared state and Biscuit handling
//! land with the P3 live tracer-bullet.

/// Stable crate identity used by early wiring tests.
pub const CRATE_NAME: &str = "presto-core";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn workspace_member_links() {
        assert_eq!(CRATE_NAME, "presto-core");
    }
}
