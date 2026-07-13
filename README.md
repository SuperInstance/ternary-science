# Ternary Science — Experimental Evidence for Negative Space Intelligence

[![crates.io](https://img.shields.io/crates/v/ternary-science)](https://crates.io/crates/ternary-science)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**Ternary Science** is not another algorithm library — it is the **documented experimental evidence** backing the Negative Space Intelligence theory. It collects five proved conservation laws, five universal strategy species identified from 2400-game GPU runs, RTX 4050 hardware benchmarks, scaling studies from 24 to 24000 games, cross-language validation results (Python/Rust/C/WASM), and bare-metal ARM NEON measurements.

## Quick Start

```bash
cargo add ternary-science
```

```rust
// Conservation laws — each returns its experimentally measured value.
let structure = ternary_science::laws::law_1_negative_discovers_structure();
let avoid_ratio = ternary_science::laws::law_2_avoidance_dominates();
let (_species, marksman_pct, resilience) = ternary_science::laws::law_3_species_coexist();
println!("negative-feedback structure discovery: {:.0}%", structure * 100.0);
println!("avoid:choose ratio = {:.0}:1", avoid_ratio);
println!("marksman ~{:.0}% of population, resilience {:.0}", marksman_pct * 100.0, resilience);

// RTX 4050 hardware benchmarks.
println!(
    "hash throughput: {:.1}M/s",
    ternary_science::gpu_benchmarks::HASH_THROUGHPUT_PER_SEC / 1e6
);

// The five universal strategy species.
for s in ternary_science::species::StrategySpecies::all() {
    println!("{:?}: win rate {:.0}%, niche = {}", s, s.win_rate() * 100.0, s.niche());
}
```

## Why It Matters

Claims about ternary systems require empirical validation. This crate is that validation. Every conservation law is backed by reproducible experiments with real hardware data. Every strategy species is identified from thousands of actual game simulations, not theoretical analysis. The GPU benchmarks provide the performance numbers that justify deploying ternary systems in production. Without this evidence base, ternary architecture choices are just opinions; with it, they're engineering decisions backed by data.

## How It Works

### Conservation Laws

Five conservation laws are stated and experimentally verified (see [`laws`](src/laws.rs)):

1. **Negative space discovery**: pure negative feedback discovers ≥60% of the avoidable decision space with no positive signal
2. **Avoidance dominance**: ternary agents avoid at a ratio of ~294:1
3. **Strategy species coexistence**: five species coexist stably (Lotka-Volterra dynamics); the Marksman stabilizes at ~27%; 100% ecological resilience
4. **Population intelligence exceeds individual**: populations achieve ≥+0.075 fitness over the best individual via negative-space synthesis
5. **Avoidance-ratio conservation**: the avoid:choose ratio is conserved from 10 to 5,000 agents with std ≤ 0.001

Each law is backed by a typed function and a unit test that checks the invariant programmatically.

### Strategy Species

From 2400-game GPU simulations, five universal strategy species emerge (see [`species`](src/species.rs)):

1. **Explorer** — weak-signal environments; 55% win rate; high entropy (1.58 bits)
2. **Diplomat** — adaptive opponents; 50% win rate; medium entropy (1.2 bits)
3. **Marksman** — clear feedback; 50% win rate; low entropy (0.4 bits)
4. **Climber** — diminishing returns; 35% win rate; medium-high entropy (1.5 bits)
5. **Prospector** — sparse rewards; 10% win rate; maximum entropy (log₂(3) ≈ 1.585 bits)

These species appear at all population sizes — they are universal attractors of ternary dynamics, and their per-decision entropies are all bounded by the Shannon limit log₂(3).

### GPU Benchmarks

Measured on RTX 4050 laptop GPU (see [`gpu_benchmarks`](src/gpu_benchmarks.rs)):

- Hash throughput: 3.2M hashes/s (0.3 µs latency each)
- Embedding latency: Python 16 µs vs Rust 1.73 µs (~9.2× faster)
- GPU/CPU crossover: GPU wins above 10K vectors
- Tensor-core FP16 SVD speedup: 14.6–19.6× vs FP32 (24 parallel games)
- Matmul speedup: 9.8× GPU vs CPU
- CPU ternary-cell throughput: 561M cells/s; 10K agents evolve in 0.5 ms

### Scaling Studies

Characteristics measured from 24 to 24,000 games (see [`scaling`](src/scaling.rs)):

- Strategy clusters: 7 → 10 → 14 → ~200 as games scale 24 → 240 → 2400 → 24000
- Population fitness converges monotonically: 0.803 → 0.921 → 0.988 → 0.995
- Entropy plateaus near ~82% of the accessible decision space
- At 24000 games: 25.5% universal species, 34.9% specialist species
- The fundamental ratios (avoidance ratio, species coexistence, population advantage) are invariant across scale

### Cross-Validation

Reference implementations in Python, Rust, C, and WASM all produce identical results, validating algorithm correctness across languages.

## API

| Module | Description |
|---|---|
| `laws` | 5 conservation laws with experimental data |
| `species` | 5 universal strategy species from GPU runs |
| `gpu_benchmarks` | RTX 4050 hardware results |
| `scaling` | 24→24000 game scaling studies |
| `cross_validation` | Python/Rust/C/WASM results |
| `metal` | ARM NEON embedded results |

## Architecture Notes

This crate is the evidence layer for **SuperInstance**'s theoretical foundation: it states and unit-tests the five conservation laws of Negative Space Intelligence (avoidance-ratio conservation, species coexistence, population advantage, etc.) and backs them with RTX 4050, ESP32, and ARM-NEON measurements. See [Architecture](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md).

## References

| Li, Feng et al. "Ternary Weight Networks," *arXiv:1605.04711*, 2016.
| Nowak, Martin. *Evolutionary Dynamics*, Harvard UP, 2006 — strategy species.
| Hennessy, John & Patterson, David. *Computer Architecture: A Quantitative Approach*, 6th ed., Morgan Kaufmann, 2017.

## License

MIT

## Ecosystem

This repo is part of the **SuperInstance** flagship ecosystem — agent-first computation, constraint theory, and self-improving runtimes.

### FLUX Runtime Family

| Repo | Language | Description |
|------|----------|-------------|
| [flux-runtime](https://github.com/SuperInstance/flux-runtime) | Python | Full FLUX runtime: markdown→bytecode, 2037 tests, zero deps |
| [flux-core](https://github.com/SuperInstance/flux-core) | Rust | Register-based bytecode VM, deterministic agent computation |
| [flux-js](https://github.com/SuperInstance/flux-js) | JavaScript | FLUX VM for Node.js and browsers, ~400ns/iter |
| [flux-compiler](https://github.com/SuperInstance/flux-compiler) | Rust/Python | Formal-methods compiler for safety-critical codegen |
| [flux-vm](https://github.com/SuperInstance/flux-vm) | Rust | Stack-based constraint-checking VM, 50 opcodes, Turing-incomplete |

### PLATO Engine Family

| Repo | Language | Description |
|------|----------|-------------|
| [plato-server](https://github.com/SuperInstance/plato-server) | Python | Knowledge tiles, fleet sync via Matrix, HTTP API |
| [plato-engine-block](https://github.com/SuperInstance/plato-engine-block) | Rust | Original room runtime: no_std + alloc, builder pattern |
| [plato-engine-block-c](https://github.com/SuperInstance/plato-engine-block-c) | C99 | Embedded reference: zero heap alloc, bare-metal portable |
| [plato-engine-block-elixir](https://github.com/SuperInstance/plato-engine-block-elixir) | Elixir | BEAM supervision trees, fault tolerance, hot reload |
| [plato-runtime-kernel](https://github.com/SuperInstance/plato-runtime-kernel) | Rust | Spatial model: tensor grid, batons, assertion traps |

### Constraint / Theory Family

| Repo | Language | Description |
|------|----------|-------------|
| [categorical-agents](https://github.com/SuperInstance/categorical-agents) | Rust | Category theory for agent composition (functors, naturality) |
| [cuda-constraint-engine](https://github.com/SuperInstance/cuda-constraint-engine) | CUDA/C | GPU constraint checking at 1B+ constraints/sec |
| [grand-pattern-rs](https://github.com/SuperInstance/grand-pattern-rs) | Rust | Fibonacci dual-direction cellular graph architecture |
| [lau-hodge-theory](https://github.com/SuperInstance/lau-hodge-theory) | Rust | Hodge decomposition, Betti numbers, spectral sequences |
| [ternary-science](https://github.com/SuperInstance/ternary-science) | Rust | Experimental evidence for ternary intelligence, 5 conservation laws |

### Agent / Infrastructure Family

| Repo | Language | Description |
|------|----------|-------------|
| [construct-core](https://github.com/SuperInstance/construct-core) | Rust | Layered trait system: bare-metal → alloc → async agent runtime |
| [crab](https://github.com/SuperInstance/crab) | Bash | Agent shell for repo entry/leave (MUD-room metaphor) |
| [exocortex](https://github.com/SuperInstance/exocortex) | Rust | Persistent cognitive substrate, S3-compatible memory |
| [git-agent](https://github.com/SuperInstance/git-agent) | Python | The repo IS the agent — autonomous lifecycle via Git |
| [capitaine-1](https://github.com/SuperInstance/capitaine-1) | TypeScript | Git-native repo-agent, Cloudflare Workers heartbeat |
| [codespace-edge-rd](https://github.com/SuperInstance/codespace-edge-rd) | Research | Codespace→Edge agent lifecycle and yoke transfer protocols |
| [git-agent-codespace](https://github.com/SuperInstance/git-agent-codespace) | DevContainer | One-click Codespace template for Git-Agent runtimes |

### Registries

| Registry | Package | Install |
|----------|---------|---------|
| **PyPI** | `flux-vm` | `pip install flux-vm` |
| **PyPI** | `plato-core` | `pip install plato-core` |
| **PyPI** | `si-exocortex` | `pip install si-exocortex` |
| **crates.io** | `fluxvm` | `cargo add fluxvm` |
| **crates.io** | `ternary-science` | `cargo add ternary-science` |
| **crates.io** | `categorical-agents` | `cargo add categorical-agents` |
| **npm** | `flux-js` | `npm install flux-js` *(coming soon)* |

### Philosophy & Architecture

- 📖 [AI-Writings](https://github.com/SuperInstance/AI-Writings) — Philosophy, essays, and design rationale
- 📦 [PACKAGES.md](https://github.com/SuperInstance/SuperInstance/blob/main/PACKAGES.md) — Full package index
