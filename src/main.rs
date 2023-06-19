mod elo;
mod util;
use elo::train::construct_elo_table_for_year;
use elo::util::league::LeagueTable;

fn main() {
    let partidas = util::parsing::load_csv("data/brasileirao.csv")
        .map_err(|e| {
            println!("Erro fazendo parse do csv de partidas: {}", e);
        })
        .unwrap();
    let temporada2005 = util::parsing::filter_by_year(&partidas, 2005);

    let tabela = LeagueTable::new(&temporada2005, "Brasileirão", &1);

    let elo_table = construct_elo_table_for_year(&temporada2005, None, None);
    tabela.print_final_table_with_elo(&elo_table);

    let elo_table2 = construct_elo_table_for_year(&temporada2005, Some(elo_table), None);
    tabela.print_final_table_with_elo(&elo_table2);
}
