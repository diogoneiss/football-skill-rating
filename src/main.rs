mod elo;
mod util;
mod experimentation;


use elo::train::{construct_elo_table_for_time_series};
use elo::util::league::LeagueTable;

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

    let elo_table_after_5_years = construct_elo_table_for_time_series(partidas, None, 2003, 2008);

    let run_config = run_config::RunConfig::default();
    let experiment_config = run_config::RunHyperparameters::default();
    let random_seed = 42;

    let (elo_simulated, simulated_matches)  = simulate_season(&temporada2009, &elo_table_after_5_years, run_config, experiment_config, random_seed);
    
    let tabela_fake = LeagueTable::new(&simulated_matches, "Brasileirão", &1);

    tabela.print_final_table();
    println!("--------------- Elo simulated ----------- \n");
    tabela_fake.print_final_table();

}
