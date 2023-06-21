use serde::{Deserialize, Serialize};

// this struct holds the necessary parameters for configuring the runtime of our experiments
// It is also used as the genotype, as it holds all the experimentation parameters
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct RunConfig {
    pub k_factor: u32,
}

impl Default for RunConfig {
    fn default() -> Self {
        RunConfig {
            k_factor: 20,
        }
    }
}
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]

pub struct RunHyperparameters {
    pub starting_elo: u32,
    pub backtest_years: u32,
    pub random_variations: u32,
    pub use_goals_diff: bool,
    pub use_home_advantage: bool,
    pub use_market_values: bool,
    pub leagues_to_use: u32,
}

impl Default for RunHyperparameters {
    fn default() -> Self {
        RunHyperparameters {
            starting_elo: 1000,
            backtest_years: 8,
            use_goals_diff: false,
            use_home_advantage: false,
            use_market_values: false,
            leagues_to_use: 1,
            random_variations: 20,
        }
    }
}