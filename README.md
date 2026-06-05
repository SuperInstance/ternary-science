# ternary-science

**Negative Space Intelligence — Experimental Evidence**

This crate is not another algorithm library. It is the **documented experimental
evidence** backing the Negative Space Intelligence theory. Every conservation law,
every GPU benchmark, every cross-language validation result — collected in one
place with real numbers, real hardware, and reproducible tests.

## What is Negative Space Intelligence?

The core insight: **the intelligence lives in what you *don't* do.** In a ternary
decision space (-1, 0, +1), the negative state carries more information density
than the positive state. Agents learn primarily through avoidance, at a ratio of
294:1 over active choice. This isn't a preference — it's a structural property
of ternary information geometry.

## The Five Proved Laws

| # | Law | Key Evidence |
|---|-----|-------------|
| 1 | Negative space discovers hidden structure | 60% avoidance from negative feedback alone |
| 2 | Avoidance dominates choice | 294:1 avoid:choose ratio |
| 3 | Strategy species coexist stably | Lotka-Volterra dynamics, 100% resilience |
| 4 | Population > individual | +0.075 fitness advantage |
| 5 | Avoidance ratio conserved across scales | std = 0.001 from 10 to 5000 agents |

## Hardware Benchmarks (RTX 4050)

- **Hash**: 3.2M/s (0.3 µs latency)
- **Embed**: 1.73 µs Rust (9.2× faster than Python's 16 µs)
- **GPU crossover**: 10K vectors (CPU wins below, GPU wins above)
- **Tensor cores**: FP16 14.6–19.6× faster than FP32 SVD
- **Matmul**: 9.8× GPU speedup
- **CPU throughput**: 561M cells/sec, 10K agents evolve in 0.5 ms

## Bare Metal

- **ESP32**: 279 bytes total state, 8 ns lookup
- **ARM NEON**: C beats Rust 17.5× on gate pipeline
- **ESP8266**: 8 ns compiled-policy lookup
- **Carapace hash**: 128 ns

## Scaling

| Games | Clusters | Fitness | Notes |
|-------|----------|---------|-------|
| 24    | 7        | 0.803   | Initial emergence |
| 240   | 10       | 0.921   | Structure forming |
| 2,400 | 14       | 0.988   | Near convergence |
| 24,000| 200      | 0.995   | Full speciation: 25.5% universal, 34.9% specialist |

## Cross-Validation

All test vectors unified across four languages:

| Language | Tests | Status |
|----------|-------|--------|
| Python   | 16/16 | ✅ |
| Rust     | 5/5   | ✅ |
| C        | 19/19 | ✅ |
| WASM     | 17/17 | ✅ |

3 divergences found and fixed (BLAKE3→BLAKE2b, two BLAKE2b-64→128 truncation fixes).

## Running the Tests

```bash
cargo test
```

30+ tests that verify every experimental claim programmatically. If the tests pass,
the evidence holds.

## License

MIT

## See Also
- **ternary-experiment** — related
- **ternary-benchmark** — related
- **ternary-fitness** — related
- **ternary-validation** — related
- **ternary-entropy** — related

