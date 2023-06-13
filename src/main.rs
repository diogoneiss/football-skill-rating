// #![allow(unused)]

use crate::prelude::*;

mod prelude;
mod data_structures;
mod parsing;
mod error;
// reference utils/season.rs
mod utils;
use utils::season::construct_seasons;

//reference elo/train.rs
mod elo;
use elo::train::{construct_elo_table_for_year, construct_elo_table_for_time_series};

fn main() {
    

    let partidas = parsing::load_csv("./src/brasileirao.csv").or_else(|e| {
        println!("Erro fazendo parse do csv de partidas: {}", e);
        Err(e)
    }).unwrap();

    let temporada2005 = parsing::filter_by_year(&partidas, 2005);

    let tabela = data_structures::LeagueTable::new(&temporada2005);
    let classificacoes = tabela.rank();


    let seasons = construct_seasons(partidas.clone());
    let desired_season = seasons.get(&2005).unwrap();
    println!("{:?} {:?} {:?} {:?}", desired_season.year, desired_season.division, desired_season.campeonato, desired_season.matches.len());
    

    // for year in seasons.keys() {
    //     println!("{}: {}", year, seasons.get(year).unwrap().matches.len());

    //     data_structures::LeagueTable::new(&seasons.get(year).unwrap().matches).print_final_table();
    // }

    //tabela.print_final_table();

    let season_years = utils::season::get_seasons_in_season_map(&seasons);
    println!("{:?}", season_years);

    let all_teams = utils::season::get_all_teams_in_season_map(&seasons);

    println!("{:?}", all_teams);
    /*
    let elo_table = construct_elo_table_for_year(&temporada2005, None);
    tabela.print_final_table_with_elo(&elo_table);

    let elo_table2 = construct_elo_table_for_year(&temporada2005, Some(elo_table));
    tabela.print_final_table_with_elo(&elo_table2);
    */

    construct_elo_table_for_time_series(partidas, 2003, 2015)

}
