
use crate::elo::data_structures::EloTable;

pub fn print_elo_table(elo_table: &EloTable) {
    let max_team_length = elo_table
        .keys()
        .map(|team| team.len())
        .max()
        .unwrap_or(0);

    let max_elo_length = elo_table
        .values()
        .map(|elo| format!("{:.2}", elo.rating).len())
        .max()
        .unwrap_or(0);

    let divider_length = max_team_length + max_elo_length + 5; // Adding 5 to account for the extra characters in the format

    let divider = "-".repeat(divider_length);
    
    println!("{}", divider);
    for (team, elo) in elo_table {
        let elo_string = format!("{:.2}", elo.rating);
        println!("| {:<max_team_width$} : {:<max_elo_width$} |", team, elo_string, max_team_width = max_team_length, max_elo_width = max_elo_length);
    }
    println!("{}", divider);
}