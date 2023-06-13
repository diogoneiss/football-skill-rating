
use skillratings::{
    elo::{elo, EloConfig, EloRating},
    Outcomes,
};

use std::collections::HashMap;

pub type RankedMatch = (EloRating, Outcomes);
pub type EloTable = HashMap<String, EloRating>;