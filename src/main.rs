use std::{collections::HashMap, env};

use problems::{day_2::Day2, utils::read_input_to_lines};

use crate::problems::{day_1::Day1, utils::Day};

#[macro_use]
extern crate fstrings;

mod problems;

fn main() {
    let solutions: HashMap<&str, Box<dyn Day>> = HashMap::from([
        ("1", Box::new(Day1 {}) as Box<dyn Day>),
        ("2", Box::new(Day2 {}) as Box<dyn Day>),
    ]);
    let args: Vec<String> = env::args().collect();
    let day = &args[1];
    let part = &args[2];
    let input = read_input_to_lines(day);
    match part.as_ref() {
        "1" => {
            let solution = solutions.get(day.as_str()).unwrap();
            println!("{}", solution.part_1(input));
        }
        "2" => {
            let solution = solutions.get(day.as_str()).unwrap();
            println!("{}", solution.part_2(input));
        }
        _ => (),
    }
}
