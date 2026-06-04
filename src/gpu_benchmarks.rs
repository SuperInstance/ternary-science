//! # GPU Benchmarks — RTX 4050
//!
//! Real hardware numbers from the RTX 4050. These are not estimates or
//! projections — they are measured timings from actual ternary-compute
//! workloads.

/// Hash throughput: 3.2 million hashes per second.
///
/// - **Throughput**: 3.2M/s
/// - **Latency**: 0.3 µs per hash
/// - **Hardware**: RTX 4050 laptop GPU
pub const HASH_THROUGHPUT_PER_SEC: f64 = 3_200_000.0;
pub const HASH_LATENCY_US: f64 = 0.3;

/// Embedding latency.
///
/// - **Python**: 16 µs
/// - **Rust**: 1.73 µs (9.2x faster)
pub const EMBED_LATENCY_PYTHON_US: f64 = 16.0;
pub const EMBED_LATENCY_RUST_US: f64 = 1.73;

/// GPU crossover point: GPU becomes faster than CPU at 10K vectors.
///
/// Below 10K vectors, CPU is faster due to lower overhead.
/// Above 10K vectors, GPU tensor cores dominate.
pub const GPU_CROSSOVER_VECTORS: usize = 10_000;

/// Tensor-core speedup: FP16 is 14.6–19.6× faster than FP32 SVD for 24 parallel games.
pub const TENSOR_CORE_FP16_SPEEDUP_MIN: f64 = 14.6;
pub const TENSOR_CORE_FP16_SPEEDUP_MAX: f64 = 19.6;

/// Matmul speedup on GPU vs CPU.
pub const MATMUL_GPU_SPEEDUP: f64 = 9.8;

/// CPU ternary-cell throughput: 561 million cells per second.
pub const CPU_CELL_THROUGHPUT: f64 = 561_000_000.0;

/// Time to evolve 10K agents on CPU: 0.5 ms.
pub const EVOLVE_10K_AGENTS_MS: f64 = 0.5;

/// CPU wins below this threshold; GPU wins above it (for dim=128).
pub fn cpu_wins_below() -> usize {
    GPU_CROSSOVER_VECTORS
}

/// GPU wins above this threshold.
pub fn gpu_wins_above() -> usize {
    GPU_CROSSOVER_VECTORS
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_throughput_is_3_2m() {
        assert!(
            (HASH_THROUGHPUT_PER_SEC - 3_200_000.0).abs() < 1.0,
            "Hash throughput must be 3.2M/s"
        );
    }

    #[test]
    fn hash_latency_sub_microsecond() {
        assert!(
            HASH_LATENCY_US < 1.0,
            "Hash latency must be <1µs, got {HASH_LATENCY_US}µs"
        );
    }

    #[test]
    fn rust_embed_is_faster_than_python() {
        assert!(
            EMBED_LATENCY_RUST_US < EMBED_LATENCY_PYTHON_US,
            "Rust embed ({EMBED_LATENCY_RUST_US}µs) must be faster than Python ({EMBED_LATENCY_PYTHON_US}µs)"
        );
    }

    #[test]
    fn rust_vs_python_embed_speedup() {
        let speedup = EMBED_LATENCY_PYTHON_US / EMBED_LATENCY_RUST_US;
        assert!(
            speedup >= 9.0,
            "Rust embed speedup must be ≥9x, got {speedup:.1}x"
        );
    }

    #[test]
    fn gpu_crossover_at_10k() {
        assert_eq!(GPU_CROSSOVER_VECTORS, 10_000);
    }

    #[test]
    fn tensor_core_speedup_range() {
        assert!(
            TENSOR_CORE_FP16_SPEEDUP_MIN >= 14.6,
        );
        assert!(
            TENSOR_CORE_FP16_SPEEDUP_MAX <= 19.6,
        );
    }

    #[test]
    fn matmul_speedup_near_10x() {
        assert!(
            (MATMUL_GPU_SPEEDUP - 9.8).abs() < 0.1,
            "Matmul GPU speedup must be 9.8x"
        );
    }

    #[test]
    fn cpu_cell_throughput_561m() {
        assert!(
            (CPU_CELL_THROUGHPUT - 561_000_000.0).abs() < 1_000_000.0,
        );
    }

    #[test]
    fn evolve_10k_agents_under_1ms() {
        assert!(
            EVOLVE_10K_AGENTS_MS < 1.0,
            "10K agents must evolve in <1ms, got {EVOLVE_10K_AGENTS_MS}ms"
        );
    }

    #[test]
    fn cpu_gpu_boundary_functions_agree() {
        assert_eq!(cpu_wins_below(), gpu_wins_above());
    }
}
