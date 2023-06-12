// #![allow(unused)]

use crate::prelude::*;

mod prelude;
mod data_structures;
mod parsing;
mod error;

fn main() {
    println!("Hello, world!");

    parsing::load_csv("./src/brasileirao.csv").unwrap();
}
