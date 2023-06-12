use crate::data_structures::Partida;
use crate::data_structures::Season;
use std::collections::HashMap;
use std::collections::HashSet;

type SeasonMap = HashMap<u16, Season>;
pub fn construct_seasons(partidas: Vec<Partida>) -> SeasonMap {
    let mut seasons: SeasonMap = HashMap::new();
    for partida in partidas {
        let year = partida.year;
        let season = seasons.entry(year).or_insert(Season::new(year));
        season.matches.push(partida);
    }
    seasons
}

pub fn get_seasons_in_season_map(seasons: &SeasonMap) -> Vec<u16> {
    let mut seasons_vec: Vec<u16> = Vec::new();
    for year in seasons.keys() {
        seasons_vec.push(*year);
    }
    seasons_vec.sort_unstable();
    seasons_vec
}

pub fn get_all_teams_in_season_map(seasons: &SeasonMap) -> Vec<String> {
    let mut teams = HashSet::new();
    for season in seasons.values() {
        for partida in &season.matches {
            teams.insert(partida.home.clone());
            teams.insert(partida.away.clone());
        }
    }
    //converter o hashset para um vetor e ordenar lexicograficamente
    let mut teams_vec: Vec<String> = teams.into_iter().collect();
    teams_vec.sort_unstable();
    teams_vec
}