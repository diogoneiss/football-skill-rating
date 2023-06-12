use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
enum MatchResult {
    H,
    A,
    D,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Partida {
    id: u64,
    #[serde(rename = "HomeTeam")]
    home: String,
    #[serde(rename = "AwayTeam")]
    away: String,
    #[serde(rename = "FTHG")]
    home_score: u8,
    #[serde(rename = "FTAG")]
    away_score: u8,
    #[serde(rename = "FTR")]
    result: MatchResult,
    #[serde(rename = "Season")]
    year: u16,
}