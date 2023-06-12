
use crate::data_structures::Partida as Partida;
use crate::data_structures::MatchResult as MatchResult;
use skillratings::Outcomes as Outcomes;

pub fn get_match_outcome(partida: &Partida) -> (Outcomes, Outcomes) {
    match partida.result {
        MatchResult::H => (Outcomes::WIN, Outcomes::LOSS),
        MatchResult::A => (Outcomes::LOSS, Outcomes::WIN),
        MatchResult::D => (Outcomes::DRAW,Outcomes::DRAW)
    }
}

pub fn get_goal_diff(partida: &Partida) -> i32 {
    partida.home_score as i32 - partida.away_score as i32
}