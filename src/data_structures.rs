use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MatchResult {
    H,
    A,
    D,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Partida {
    pub id: u64,
    #[serde(rename = "HomeTeam")]
    pub home: String,
    #[serde(rename = "AwayTeam")]
    pub away: String,
    #[serde(rename = "FTHG")]
    pub home_score: u16,
    #[serde(rename = "FTAG")]
    pub away_score: u16,
    #[serde(rename = "FTR")]
    pub result: MatchResult,
    #[serde(rename = "Season")]
    pub year: u16,
}
#[derive(Default, Debug)]
pub struct TeamStats {
    pub goals_scored: u16,
    pub goals_conceded: u16,
    pub wins: u16,
    pub draws: u16,
    pub losses: u16,
    pub played: u16,
    pub points: u16,
}

pub struct LeagueTable<'a> {
    pub table: std::collections::HashMap<&'a str, TeamStats>,
    pub year: u16,
    pub division: u8,
    pub campeonato: String,
}

impl<'a> LeagueTable<'a> {
    pub fn new(matches: &'a Vec<Partida>) -> Self {
        let mut table = std::collections::HashMap::new();

        for match_instance in matches {
            let home_name = match_instance.home.as_str();
            let away_name = match_instance.away.as_str();

            // necessario para evitar que o compilador reclame de emprestimos mutaveis no mesmo escopo
            {
                let home_stats = table.entry(home_name).or_insert(TeamStats::default());
                home_stats.played += 1;
                home_stats.goals_scored += match_instance.home_score;
                home_stats.goals_conceded += match_instance.away_score;
            }

            {
                let away_stats = table.entry(away_name).or_insert(TeamStats::default());
                away_stats.played += 1;
                away_stats.goals_scored += match_instance.away_score;
                away_stats.goals_conceded += match_instance.home_score;
            }

            match match_instance.result {
                MatchResult::H => {
                    let home_stats = table.get_mut(home_name).unwrap();
                    home_stats.wins += 1;
                    home_stats.points += 3;

                    let away_stats = table.get_mut(away_name).unwrap();
                    away_stats.losses += 1;
                }
                MatchResult::A => {
                    let home_stats = table.get_mut(home_name).unwrap();
                    home_stats.losses += 1;

                    let away_stats = table.get_mut(away_name).unwrap();
                    away_stats.wins += 1;
                    away_stats.points += 3;
                }
                MatchResult::D => {
                    let home_stats = table.get_mut(home_name).unwrap();
                    home_stats.draws += 1;
                    home_stats.points += 1;

                    let away_stats = table.get_mut(away_name).unwrap();
                    away_stats.draws += 1;
                    away_stats.points += 1;
                }
            }
        }

        LeagueTable {
            year: matches[0].year,
            division: 1,
            campeonato: String::from("Brasileirão"),
            table,
        }
    }
}

impl<'a> LeagueTable<'a> {
    pub fn rank(&self) -> Vec<(&'a str, &TeamStats)> {
        let mut teams: Vec<_> = self.table.iter().map(|(a, b)| (*a, b)).collect();

        teams.sort_by(|(a_name, a_stats), (b_name, b_stats)| {
            let a_goal_diff = a_stats.goals_scored as i32 - a_stats.goals_conceded as i32;
            let b_goal_diff = b_stats.goals_scored as i32 - b_stats.goals_conceded as i32;

            // Compare teams based on points, then goal difference, then goals scored.
            // Reverse because we want to sort in descending order.
            b_stats
                .points
                .cmp(&a_stats.points)
                .then_with(|| b_goal_diff.cmp(&a_goal_diff))
                .then_with(|| b_stats.goals_scored.cmp(&a_stats.goals_scored))
        });

        teams
    }
}
impl<'a> LeagueTable<'a> {

    pub fn print_final_table(&self) {
        let ranked_teams = self.rank();
    
        println!(
            "League Table for the year {}: Division {} of {}",
            self.year, self.division, self.campeonato
        );
    
        println!(
            "{:<5} {:<20} {:<7} {:<5} {:<5} {:<7} {:<14} {:<14} {:<8}",
            "Rank", "Team", "Points", "Wins", "Draws", "Losses", "Goals Scored", "Goals Conceded", "Played"
        );
    
        for (index, (team_name, team_stats)) in ranked_teams.iter().enumerate() {
            println!(
                "{:<5} {:<20} {:<7} {:<5} {:<5} {:<7} {:<14} {:<14} {:<8}",
                index + 1,  // Add 1 to the index because enumerate starts at 0
                team_name,
                team_stats.points,
                team_stats.wins,
                team_stats.draws,
                team_stats.losses,
                team_stats.goals_scored,
                team_stats.goals_conceded,
                team_stats.played
            );
        }
    }
}
#[derive(Debug, Clone)]
pub struct Season {
    pub year: u16,
    pub division: u8,
    pub campeonato: String,
    pub matches: Vec<Partida>,
}

impl Season {
    pub fn new(year: u16) -> Self {
        Season {
            year,
            division: 1,
            campeonato: String::from("Brasileirão"),
            matches: Vec::new(),
        }
    }
}