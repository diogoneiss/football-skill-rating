use crate::data_structures;
use crate::utils::partida::{get_match_outcome, get_goal_diff};

use skillratings::{
    elo::{elo, EloConfig, EloRating},
    Outcomes,
};

use std::collections::HashMap;

pub type RankedMatch = (EloRating, Outcomes);
pub type EloTable = HashMap<String, EloRating>;
pub fn construct_elo_table(partidas: &Vec<data_structures::Partida>, starting_elos: Option<EloTable>) -> EloTable {
    //construir tabela de elo se vier vazia
    let mut elo_table = match starting_elos {
        Some(elos) => elos,
        None => HashMap::new(),
    };
    
    let mut results_table: HashMap<String, Vec<RankedMatch>> = HashMap::new();

    // salvar historico de elo desses times
    for partida in partidas {
        let home_team = partida.home.clone();
        let away_team = partida.away.clone();
        
        let (home_outcome, away_outcome) = get_match_outcome(partida);

        let current_elo = |team_name: &String| {
            elo_table.get(team_name)
            .cloned()
            .unwrap_or_else(|| {
                EloRating::new() 
            })
        };

        let home_team_elo = current_elo(&home_team);
        let away_team_elo = current_elo(&away_team);

        // salvar historico de resultados desses times e elos
        let mut insert_result = |team_name: &String, current_elo: EloRating, outcome | {
            results_table.entry(team_name.clone()).or_insert(Vec::new()).push((current_elo, outcome));
        };

        insert_result(&home_team, home_team_elo, home_outcome);
        insert_result(&away_team, away_team_elo, away_outcome);


        let (new_player_home, new_player_away) = elo(&home_team_elo, &away_team_elo, &home_outcome, &EloConfig::new());

        if home_team == "Cruzeiro" || away_team == "Cruzeiro" {
            println!("{:?}", partida);
            if home_team == "Cruzeiro"{
                println!("Cruzeiro: elo: {} -> {}", home_team_elo.rating, new_player_home.rating);
            }
            else {
                println!("Cruzeiro: elo: {} -> {}", away_team_elo.rating, new_player_away.rating);
            }

        }

        elo_table.insert(home_team, new_player_home);
        elo_table.insert(away_team, new_player_away);
    }
    elo_table
}