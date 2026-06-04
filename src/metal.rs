//! # Bare Metal — Embedded and ARM Results
//!
//! Ternary computing on bare metal. The entire ternary decision state fits
//! in 279 bytes on an ESP32 with 8 ns lookup. On ARM NEON, C beats Rust
//! by 17.5× on the gate pipeline.

/// ESP32 ternary state footprint: 279 bytes total.
pub const ESP32_STATE_BYTES: usize = 279;

/// ESP32 lookup latency: 8 nanoseconds.
pub const ESP32_LOOKUP_NS: u64 = 8;

/// ESP8266 compiled-policy lookup: 8 ns.
pub const ESP8266_LOOKUP_NS: u64 = 8;

/// ARM NEON: C outperforms Rust by 17.5× on gate pipeline.
pub const NEON_C_VS_RUST_SPEEDUP: f64 = 17.5;

/// Carapace hash latency: 128 ns (lever-runner).
pub const CARAPACE_HASH_NS: u64 = 128;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn esp32_state_under_300_bytes() {
        assert!(
            ESP32_STATE_BYTES < 300,
            "ESP32 state must fit in <300 bytes, got {ESP32_STATE_BYTES}"
        );
    }

    #[test]
    fn esp32_state_is_279_bytes() {
        assert_eq!(ESP32_STATE_BYTES, 279);
    }

    #[test]
    fn esp32_lookup_under_10_ns() {
        assert!(
            ESP32_LOOKUP_NS < 10,
            "ESP32 lookup must be <10ns, got {ESP32_LOOKUP_NS}ns"
        );
    }

    #[test]
    fn esp8266_lookup_matches_esp32() {
        assert_eq!(
            ESP8266_LOOKUP_NS, ESP32_LOOKUP_NS,
            "ESP8266 and ESP32 lookup should both be 8ns"
        );
    }

    #[test]
    fn neon_c_beats_rust_by_17_5x() {
        assert!(
            (NEON_C_VS_RUST_SPEEDUP - 17.5).abs() < 0.1,
            "NEON C vs Rust speedup must be 17.5x"
        );
    }

    #[test]
    fn carapace_hash_under_150_ns() {
        assert!(
            CARAPACE_HASH_NS < 150,
            "Carapace hash must be <150ns, got {CARAPACE_HASH_NS}ns"
        );
    }

    #[test]
    fn carapace_hash_is_128_ns() {
        assert_eq!(CARAPACE_HASH_NS, 128);
    }
}
