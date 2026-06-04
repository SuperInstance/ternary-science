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

pub mod laws;
pub mod species;
pub mod gpu_benchmarks;
pub mod scaling;
pub mod cross_validation;
pub mod metal;
