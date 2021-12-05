use super::utils::Day;

pub struct Day2;
impl Day for Day2 {
    fn part_1(&self, input: Vec<String>) -> String {
        let mut front = 0;
        let mut depth = 0;
        input
            .iter()
            .map(parse_line_and_move)
            .filter_map(|e| e)
            .for_each(|direction| match direction {
                Direction::Up(n) => depth -= n,
                Direction::Down(n) => depth += n,
                Direction::Forward(n) => front += n,
            });
        (front * depth).to_string()
    }

    fn part_2(&self, input: Vec<String>) -> String {
        let mut front = 0;
        let mut depth = 0;
        let mut aim = 0;
        input
            .iter()
            .map(parse_line_and_move)
            .filter_map(|e| e)
            .for_each(|direction| match direction {
                Direction::Up(n) => aim -= n,
                Direction::Down(n) => aim += n,
                Direction::Forward(n) => {
                    front += n;
                    depth += aim * n;
                }
            });
        (front * depth).to_string()
    }
}

enum Direction {
    Forward(i64),
    Down(i64),
    Up(i64),
}

fn parse_line_and_move(line: &String) -> Option<Direction> {
    let line = line.split(" ").collect::<Vec<&str>>();
    match line.as_slice() {
        ["forward", n] => Some(Direction::Forward(n.parse::<i64>().unwrap())),
        ["down", n] => Some(Direction::Down(n.parse::<i64>().unwrap())),
        ["up", n] => Some(Direction::Up(n.parse::<i64>().unwrap())),
        _ => None,
    }
}
