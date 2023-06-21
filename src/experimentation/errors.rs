use super::run_config;


pub fn print_errors_by_year(errors: &Vec<f64>, experiment_config: &run_config::RunHyperparameters) {
    let base_year = experiment_config.backtest_years + experiment_config.starting_year + 1;
    
    let horizontal_line = format!("{:-<1$}", "", 19);
    println!("{}", &horizontal_line);
    println!("|Errors by year:  |");

    for (i, error) in errors.iter().enumerate() {
        println!("|{}  :  {:.2}   |", base_year+i as u16, error);
    }
    println!("{}", &horizontal_line);

}

