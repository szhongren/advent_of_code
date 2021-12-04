use std::fs::read_to_string;

pub fn read_input_to_lines(day: &String) -> Vec<String> {
    read_to_string(f!("inputs/{day}.txt"))
        .expect("Something went wrong reading the file")
        .clone()
        .lines()
        .map(|x| x.trim())
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
}

pub trait Day {
    fn part_1(&self, input: Vec<String>) -> String;
    fn part_2(&self, input: Vec<String>) -> String;
}
