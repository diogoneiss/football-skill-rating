// #![allow(unused)]

use crate::prelude::*;

mod prelude;
mod data_structures;
mod parsing;
mod error;

fn main() {
    println!("Hello, world!");

    let partidas = parsing::load_csv("./src/brasileirao.csv").or_else(|e| {
        println!("Erro fazendo parse do csv de partidas: {}", e);
        Err(e)
    }).unwrap();

    let temporada2005 = parsing::filter_by_year(&partidas, 2005);

    let mut tabela = data_structures::LeagueTable::new(&temporada2005);
    let classificacoes = tabela.rank();
    tabela.print_final_table();
}
