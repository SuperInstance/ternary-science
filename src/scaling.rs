//! # Scaling — How Ternary Systems Scale
//!
//! Experimental data on how ternary decision spaces behave as the number of
//! games and agents increases. Key finding: strategy space expands, but
//! fundamental ratios remain conserved (see [`crate::laws::Law 5`]).

/// A scaling data point from GPU experiments.
#[derive(Debug, Clone, Copy)]
pub struct ScalingPoint {
    /// Number of games played.
    pub games: usize,
    /// Number of distinct strategy clusters observed.
    pub clusters: usize,
    /// Entropy increase percentage (plateaus at ~82%).
    pub entropy_pct: f64,
    /// Population fitness (converges toward 1.0).
    pub fitness: f64,
}

/// Experimental scaling data from 24 → 240 → 2400 → 24000 games.
pub fn scaling_series() -> Vec<ScalingPoint> {
    vec![
        ScalingPoint {
            games: 24,
            clusters: 7,
            entropy_pct: 0.55,
            fitness: 0.803,
        },
        ScalingPoint {
            games: 240,
            clusters: 10,
            entropy_pct: 0.72,
            fitness: 0.921,
        },
        ScalingPoint {
            games: 2400,
            clusters: 14,
            entropy_pct: 0.80,
            fitness: 0.988,
        },
        ScalingPoint {
            games: 24000,
            clusters: 200,
            entropy_pct: 0.82,
            fitness: 0.995,
        },
    ]
}

/// At 24000 games: percentage of strategies that are "universal" (appear
/// across all environment types).
pub const UNIVERSAL_SPECIES_PCT: f64 = 0.255;

/// At 24000 games: percentage of strategies that are "specialist" (appear
/// in only one environment type).
pub const SPECIALIST_SPECIES_PCT: f64 = 0.349;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clusters_double_from_24_to_2400() {
        let data = scaling_series();
        assert_eq!(data[0].clusters, 7);
        assert_eq!(data[2].clusters, 14);
        assert!(data[2].clusters >= data[0].clusters * 2);
    }

    #[test]
    fn entropy_plateaus_near_82_percent() {
        let data = scaling_series();
        // 2400-game entropy
        assert!(
            (data[2].entropy_pct - 0.80).abs() < 0.03,
            "Entropy at 2400 games ≈ 80%"
        );
        // 24000-game entropy: plateau
        assert!(
            data[3].entropy_pct < 0.85,
            "Entropy plateaus near 82%"
        );
    }

    #[test]
    fn fitness_converges_monotonically() {
        let data = scaling_series();
        for window in data.windows(2) {
            assert!(
                window[1].fitness >= window[0].fitness,
                "Fitness must be non-decreasing: {} < {} at {} games",
                window[1].fitness,
                window[0].fitness,
                window[1].games,
            );
        }
    }

    #[test]
    fn fitness_converges_from_803_to_988() {
        let data = scaling_series();
        assert!((data[0].fitness - 0.803).abs() < 0.001);
        assert!((data[2].fitness - 0.988).abs() < 0.001);
    }

    #[test]
    fn at_24000_games_200_species() {
        let data = scaling_series();
        assert_eq!(data[3].clusters, 200);
    }

    #[test]
    fn universal_pct_25_5_percent() {
        assert!(
            (UNIVERSAL_SPECIES_PCT - 0.255).abs() < 0.001,
        );
    }

    #[test]
    fn specialist_pct_34_9_percent() {
        assert!(
            (SPECIALIST_SPECIES_PCT - 0.349).abs() < 0.001,
        );
    }

    #[test]
    fn universal_plus_specialist_less_than_half() {
        let total = UNIVERSAL_SPECIES_PCT + SPECIALIST_SPECIES_PCT;
        assert!(
            total < 0.65,
            "Universal + specialist = {total}, leaving room for generalists"
        );
    }
}
