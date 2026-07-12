//! # Universal Strategy Species
//!
//! Five strategy species that emerge universally from ternary agents playing
//! 2400 games on GPU hardware. These are not designed — they *emerge* from the
//! interaction of ternary decision spaces with different environment types.

/// Maximum possible Shannon entropy of a single ternary decision over the
/// alphabet {-1, 0, +1}, in bits.
///
/// # Derivation
///
/// For a distribution (p₋₁, p₀, p₊₁) with Σ pᵢ = 1, the Shannon entropy
/// H = −Σ pᵢ log₂ pᵢ is maximized when the distribution is uniform,
/// p = (1/3, 1/3, 1/3) (Gibbs' inequality / strict concavity of log₂). There
///
/// ```text
/// H_max = −3 · (1/3) · log₂(1/3) = log₂(3) = ln 3 / ln 2 ≈ 1.5850 bits.
/// ```
///
/// Therefore *no* per-decision ternary entropy may exceed this value. A
/// reported entropy above log₂(3) is mathematically impossible and indicates a
/// units/data error. (`std` exposes no `LN_3`, so the value is hardcoded here
/// and cross-checked against `3.0f64.log2()` in the test
/// `ternary_max_entropy_constant_is_log2_of_3`.)
pub const TERNARY_MAX_ENTROPY_BITS: f64 = 1.584_962_500_721_156;

/// A universal strategy species identified in 2400-game GPU experiments.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StrategySpecies {
    /// **Explorer** — thrives in weak-signal environments.
    ///
    /// Win rate: 55%. High entropy exploration. When the environment gives
    /// little feedback, Explorer spreads its avoidance widely and discovers
    /// structure by elimination.
    Explorer,

    /// **Diplomat** — adapts to opponent behavior.
    ///
    /// Win rate: 50%. Medium entropy. Excels against adaptive opponents by
    /// mirroring their avoidance patterns and exploiting the negative space
    /// they leave open.
    Diplomat,

    /// **Marksman** — exploits clear feedback signals.
    ///
    /// Win rate: 50%. Low entropy, high precision. When feedback is unambiguous,
    /// Marksman concentrates avoidance on confirmed bad paths and converges fast.
    Marksman,

    /// **Climber** — struggles against diminishing returns.
    ///
    /// Win rate: 35%. In environments where early gains are easy but later gains
    /// require exponentially more exploration, Climber gets trapped in local optima.
    Climber,

    /// **Prospector** — maximizes diversity in sparse-reward environments.
    ///
    /// Win rate: 10%. Maximum diversity: log₂(3) ≈ 1.585 bits — the Shannon
    /// entropy bound for a ternary decision, attained by the uniform
    /// distribution over {-1, 0, +1} (see [`TERNARY_MAX_ENTROPY_BITS`]).
    /// Prospector almost never "wins" in the traditional sense, but its
    /// exploration fills in the negative-space map for the entire population.
    Prospector,
}

impl StrategySpecies {
    /// Returns the experimentally observed win rate for this species.
    pub fn win_rate(&self) -> f64 {
        match self {
            Self::Explorer => 0.55,
            Self::Diplomat => 0.50,
            Self::Marksman => 0.50,
            Self::Climber => 0.35,
            Self::Prospector => 0.10,
        }
    }

    /// Returns the entropy level (low/medium/high) and numeric value.
    ///
    /// All values are bounded by [`TERNARY_MAX_ENTROPY_BITS`] (log₂(3) ≈ 1.585
    /// bits), the maximum Shannon entropy of a ternary decision. The previous
    /// values (Explorer 1.8, Prospector 1.99) exceeded this bound and were
    /// impossible for a per-decision ternary distribution.
    pub fn entropy(&self) -> (&'static str, f64) {
        match self {
            Self::Explorer => ("high", 1.58),
            Self::Diplomat => ("medium", 1.2),
            Self::Marksman => ("low", 0.4),
            Self::Climber => ("medium-high", 1.5),
            Self::Prospector => ("maximum", TERNARY_MAX_ENTROPY_BITS),
        }
    }

    /// Returns the environment type where this species thrives.
    pub fn niche(&self) -> &'static str {
        match self {
            Self::Explorer => "weak signal",
            Self::Diplomat => "adaptive opponents",
            Self::Marksman => "clear feedback",
            Self::Climber => "diminishing returns",
            Self::Prospector => "sparse rewards",
        }
    }

    /// All five species.
    pub fn all() -> &'static [StrategySpecies] {
        &[
            StrategySpecies::Explorer,
            StrategySpecies::Diplomat,
            StrategySpecies::Marksman,
            StrategySpecies::Climber,
            StrategySpecies::Prospector,
        ]
    }
}

/// Scaling data: how strategy clusters grow with game count.
///
/// | Games | Clusters |
/// |-------|----------|
/// | 24    | 7        |
/// | 240   | 10       |
/// | 2400  | 14       |
///
/// Win-rate standard deviation increases slightly: 0.135 → 0.149.
pub fn scaling_data() -> &'static [(usize, usize, f64)] {
    &[(24, 7, 0.135), (240, 10, 0.142), (2400, 14, 0.149)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn five_species_exist() {
        assert_eq!(StrategySpecies::all().len(), 5);
    }

    #[test]
    fn explorer_wins_55_percent() {
        assert!((StrategySpecies::Explorer.win_rate() - 0.55).abs() < 0.01,);
    }

    #[test]
    fn diplomat_wins_50_percent() {
        assert!((StrategySpecies::Diplomat.win_rate() - 0.50).abs() < 0.01,);
    }

    #[test]
    fn marksman_wins_50_percent() {
        assert!((StrategySpecies::Marksman.win_rate() - 0.50).abs() < 0.01,);
    }

    #[test]
    fn climber_wins_35_percent() {
        assert!((StrategySpecies::Climber.win_rate() - 0.35).abs() < 0.01,);
    }

    #[test]
    fn prospector_wins_10_percent() {
        assert!((StrategySpecies::Prospector.win_rate() - 0.10).abs() < 0.01,);
    }

    #[test]
    fn prospector_has_maximum_diversity() {
        let (label, bits) = StrategySpecies::Prospector.entropy();
        assert_eq!(label, "maximum");
        // "Maximum" entropy of a ternary decision is exactly the Shannon bound
        // log2(3), attained by the uniform distribution over {-1,0,+1}.
        assert!(
            (bits - TERNARY_MAX_ENTROPY_BITS).abs() < 1e-9,
            "Prospector diversity must equal log2(3) ≈ 1.585, got {bits}"
        );
    }

    #[test]
    fn entropy_respects_shannon_bound() {
        // Shannon entropy of any distribution over a 3-symbol alphabet
        // {-1, 0, +1} is bounded above by log2(3) ≈ 1.585 bits (maximum at the
        // uniform distribution). No per-decision ternary entropy may exceed it.
        for species in StrategySpecies::all() {
            let (_, bits) = species.entropy();
            assert!(
                bits <= TERNARY_MAX_ENTROPY_BITS,
                "{species:?} entropy {bits} exceeds the Shannon bound log2(3) ≈ {}",
                TERNARY_MAX_ENTROPY_BITS,
            );
        }
    }

    #[test]
    fn ternary_max_entropy_constant_is_log2_of_3() {
        // Cross-check the hardcoded bound against the standard library's
        // floating-point log2(3); guards against a typo in the constant.
        assert!(
            (TERNARY_MAX_ENTROPY_BITS - 3.0f64.log2()).abs() < 1e-12,
            "TERNARY_MAX_ENTROPY_BITS must equal log2(3), got {TERNARY_MAX_ENTROPY_BITS} vs {}",
            3.0f64.log2(),
        );
    }

    #[test]
    fn clusters_double_from_24_to_2400() {
        let data = scaling_data();
        let clusters_24 = data[0].1;
        let clusters_2400 = data[2].1;
        assert!(clusters_2400 >= clusters_24 * 2);
    }

    #[test]
    fn win_rate_std_increases_with_scale() {
        let data = scaling_data();
        assert!(data[1].2 > data[0].2, "std should increase 24→240");
        assert!(data[2].2 > data[1].2, "std should increase 240→2400");
    }

    #[test]
    fn all_species_have_distinct_niches() {
        let niches: Vec<_> = StrategySpecies::all().iter().map(|s| s.niche()).collect();
        for i in 0..niches.len() {
            for j in (i + 1)..niches.len() {
                assert_ne!(niches[i], niches[j], "Species must have distinct niches");
            }
        }
    }
}
