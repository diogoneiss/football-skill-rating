mod elo;
mod util;
mod experimentation;


use elo::train::{construct_elo_table_for_time_series};
use elo::util::league::LeagueTable;

use experimentation::compare_simulation::run_experiments;
use experimentation::run_config;
use experimentation::simulate_season::simulate_season;
fn main() {
    let partidas = util::parsing::load_csv("data/brasileirao.csv")
        .map_err(|e| {
            println!("Erro fazendo parse do csv de partidas: {}", e);
        })
        .unwrap();
    let temporada2009 = util::parsing::filter_by_year(&partidas, 2009);

    let tabela = LeagueTable::new(&temporada2009, "Brasileirão", &1);

    let elo_table_after_5_years = construct_elo_table_for_time_series(partidas.clone(), None, 2003, 2008);

    let run_config = run_config::RunConfig::default();
    let experiment_config = run_config::RunHyperparameters::default();
    

    let errors = run_experiments(&partidas , &run_config, &experiment_config);

    experimentation::errors::print_errors_by_year(&errors, &experiment_config)

}
