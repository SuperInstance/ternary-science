//! # Ternary Science — Experimental Evidence
//!
//! This crate is **not** another algorithm library. It is the **documented experimental
//! evidence** backing the Negative Space Intelligence theory: every conservation law,
//! every GPU benchmark, every cross-language validation result, collected in one place
//! with the real numbers, the real hardware, and reproducible tests.
//!
//! ## Modules
//!
//! | Module | What it proves |
//! |--------|---------------|
//! | [`laws`] | 5 proved conservation laws with experimental data |
//! | [`species`] | 5 universal strategy species from 2400-game GPU runs |
//! | [`gpu_benchmarks`] | RTX 4050 hardware benchmarks |
//! | [`scaling`] | How ternary systems scale from 24 to 24000 games |
//! | [`cross_validation`] | Cross-language (Python/Rust/C/WASM) test results |
//! | [`metal`] | Bare-metal embedded and ARM NEON results |

// This crate is an *evidence* layer: its tests intentionally assert that the
// public experimental `const` data still matches its documented values (e.g.
// `assert_eq!(GPU_CROSSOVER_VECTORS, 10_000)`, `assert!(HASH_LATENCY_US < 1.0)`).
// Those assertions are compile-time-knowable by design — they are regression
// locks on the recorded data, not runtime logic — so the
// `clippy::assertions_on_constants` lint is a false positive here and is
// suppressed crate-wide. Genuine invariants are additionally covered by
// non-constant tests (see `laws` and `species` modules).
#![allow(clippy::assertions_on_constants)]

pub mod cross_validation;
pub mod gpu_benchmarks;
pub mod laws;
pub mod metal;
pub mod scaling;
pub mod species;
