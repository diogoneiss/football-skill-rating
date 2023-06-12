use crate::data_structures::Partida;
use crate::data_structures::Season;
use std::collections::HashMap;

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