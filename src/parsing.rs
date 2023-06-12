use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use std::error::Error;
use crate::data_structures::Partida as Partida;

pub fn load_csv(path: &str) -> Result<Vec<Partida>, Box<dyn Error>> {
    
    let mut rdr = csv::Reader::from_path(path)?;

    let result: Vec<Partida> = rdr.deserialize().map(|r: Result<Partida, csv::Error>| r.unwrap() ).collect();
    
    Ok(result)
}



pub fn filter_by_year(partidas: &Vec<Partida>, year: u16) -> Vec<Partida> {
    partidas.iter().filter(|p| p.year == year).cloned().collect()
}