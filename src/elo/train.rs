use crate::data_structures;
use crate::utils::partida::{get_goal_diff, get_match_outcome};
use crate::utils::season::{construct_seasons, get_seasons_in_season_map, SeasonMap};
use skillratings::{
    elo::{elo, EloConfig, EloRating},
    Outcomes,
};

use super::data_structures::{EloTable, RankedMatch};
use core::panic;
use std::collections::HashMap;

pub fn construct_elo_table_for_year(
    partidas: &Vec<data_structures::Partida>,
    starting_elos: Option<EloTable>,
    elo_config: Option<&EloConfig>,
) -> EloTable {
    let print_exemplo = true;

    //construir tabela de elo se vier vazia
    let mut elo_table = match starting_elos {
        Some(elos) => elos,
        None => HashMap::new(),
    };
    let default_config = EloConfig::default();
    let elo_config = match elo_config {
        Some(config) => config,
        None => &default_config,
    };

    let mut results_table: HashMap<String, Vec<RankedMatch>> = HashMap::new();

    // salvar historico de elo desses times
    for partida in partidas {
        let home_team = partida.home.clone();
        let away_team = partida.away.clone();

        let (home_outcome, away_outcome) = get_match_outcome(partida);

        let current_elo = |team_name: &String| {
            elo_table
                .get(team_name)
                .cloned()
                .unwrap_or_else(|| EloRating::new())
        };

        let home_team_elo = current_elo(&home_team);
        let away_team_elo = current_elo(&away_team);

        // salvar historico de resultados desses times e elos
        let mut insert_result = |team_name: &String, current_elo: EloRating, outcome| {
            results_table
                .entry(team_name.clone())
                .or_insert(Vec::new())
                .push((current_elo, outcome));
        };

        insert_result(&home_team, home_team_elo, home_outcome);
        insert_result(&away_team, away_team_elo, away_outcome);

        let (new_player_home, new_player_away) = elo(
            &home_team_elo,
            &away_team_elo,
            &home_outcome,
            elo_config,
        );

        if print_exemplo {
            if home_team == "Cruzeiro" || away_team == "Cruzeiro" {
                println!("{:?}", partida);
                if home_team == "Cruzeiro" {
                    println!(
                        "Cruzeiro: elo: {} -> {}",
                        home_team_elo.rating, new_player_home.rating
                    );
                } else {
                    println!(
                        "Cruzeiro: elo: {} -> {}",
                        away_team_elo.rating, new_player_away.rating
                    );
                }
            }
        }

        elo_table.insert(home_team, new_player_home);
        elo_table.insert(away_team, new_player_away);
    }
    elo_table
}

fn check_time_series_interval(
    match_years_vector: &Vec<u16>,
    desired_range: &std::ops::RangeInclusive<u16>,
) {
    for year in desired_range.clone().into_iter() {
        if !match_years_vector.contains(&year) {
            let error_msg = format!("Year {} not found in season map. The range was {:?} and the years present are {:?}", year, &desired_range, match_years_vector);

            panic!("{}", error_msg);
        }
    }
}

pub fn construct_elo_table_for_time_series(
    all_matches: Vec<data_structures::Partida>,
    elo_config: Option<&EloConfig>,
    start_year: u16,
    end_year: u16,
) -> EloTable {

    let default_config = EloConfig::default();
    let elo_config = match elo_config {
        Some(config) => config,
        None => &default_config,
    };

    let seasons_map: SeasonMap = construct_seasons(all_matches);

    let years_in_season_map = get_seasons_in_season_map(&seasons_map);

    //verificar se o vetor é contíguo
    let desired_range: std::ops::RangeInclusive<u16> = start_year..=end_year;

    check_time_series_interval(&years_in_season_map, &desired_range);

    let mut starting_elo_table: Option<EloTable> = None;

    for year in desired_range.into_iter() {
        let season = seasons_map.get(&year).unwrap();
        let partidas = &season.matches;
        let elo_table = construct_elo_table_for_year(partidas, starting_elo_table, Some(&elo_config));
        starting_elo_table = Some(elo_table.clone());

        println!("Elo table for year {}", year);
        super::utils::print_elo_table(&elo_table);
    }

    starting_elo_table.unwrap()
}
