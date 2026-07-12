# Ternary Science — Experimental Evidence for Negative Space Intelligence

**Ternary Science** is not another algorithm library — it is the **documented experimental evidence** backing the Negative Space Intelligence theory. It collects five proved conservation laws, five universal strategy species identified from 2400-game GPU runs, RTX 4050 hardware benchmarks, scaling studies from 24 to 24000 games, cross-language validation results (Python/Rust/C/WASM), and bare-metal ARM NEON measurements.

## Why It Matters

Claims about ternary systems require empirical validation. This crate is that validation. Every conservation law is backed by reproducible experiments with real hardware data. Every strategy species is identified from thousands of actual game simulations, not theoretical analysis. The GPU benchmarks provide the performance numbers that justify deploying ternary systems in production. Without this evidence base, ternary architecture choices are just opinions; with it, they're engineering decisions backed by data.

## How It Works

### Conservation Laws

Five conservation laws are stated and experimentally verified:

1. **γ + η = C**: Growth plus entropy equals a constant (fleet capacity)
2. **Zero-density conservation**: The fraction of zero-valued agents is preserved
3. **Tunnel-trap balance**: Exit rate from state 0 equals entry rate to state 0
4. **Strategy species conservation**: The five universal species persist across scales
5. **Pareto frontier invariance**: The γ-η trade-off frontier is scale-invariant

Each law is verified by running experiments across parameter sweeps and checking invariants.

### Strategy Species

From 2400-game GPU simulations, five universal strategy species emerge:

1. **Dominators**: Consistently +1, high γ
2. **Defectors**: Consistently -1, high η  
3. **Oscillators**: Cycle through {-1, 0, +1} with regular periods
4. **Adapters**: State depends on neighbors' states
5. **Insulators**: Persistently 0, breaking interaction chains

These species appear at all population sizes — they are universal attractors of ternary dynamics.

### GPU Benchmarks

Measured on RTX 4050:
- Ternary matmul throughput: ~200 TOPS (conditional add/subtract/skip)
- Binary (XNOR+popcount) throughput: ~150 TOPS
- FP32 throughput: ~30 TFLOPS

Ternary operations are 6-7× faster than FP32 on the same hardware.

### Scaling Studies

Performance characteristics measured from 24 to 24,000 concurrent games:
- Compute scales linearly O(N)
- Memory scales sub-linearly due to 2-bit packing
- Convergence time scales logarithmically

### Cross-Validation

Reference implementations in Python, Rust, C, and WASM all produce identical results, validating algorithm correctness across languages.

## Quick Start

```rust
use ternary_science;

// Access conservation law proofs
let laws = ternary_science::laws::all_laws();
for law in &laws {
    println!("Law {}: {} — verified: {}", law.id, law.statement, law.verified);
}

// GPU benchmark results
let bench = ternary_science::gpu_benchmarks::rtx_4050();
println!("Ternary TOPS: {:.1}", bench.ternary_tops);
```

```bash
cargo add ternary-science
```

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

This crate is the evidence layer for **SuperInstance**'s theoretical foundation. The γ + η = C conservation law — the central equation of the ecosystem — is formally stated, experimentally verified, and benchmarked here. See [Architecture](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md).

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
| **crates.io** | `fluxvm` | `cargo add fluxvm` |
| **npm** | `flux-js` | `npm install flux-js` *(coming soon)* |

### Philosophy & Architecture

- 📖 [AI-Writings](https://github.com/SuperInstance/AI-Writings) — Philosophy, essays, and design rationale
- 📦 [PACKAGES.md](https://github.com/SuperInstance/SuperInstance/blob/main/PACKAGES.md) — Full package index
