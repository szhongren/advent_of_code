use super::utils::Day;

pub struct Day1;
impl Day for Day1 {
    fn part_1(&self, input: Vec<String>) -> String {
        let values = self.parse_input(input);
        let previous_values = &values.clone()[..values.len()];
        let next_values = &values.clone()[1..];
        previous_values
            .iter()
            .zip(next_values.iter())
            .filter(|(a, b)| a < b)
            .count()
            .to_string()
    }

    fn part_2(&self, input: Vec<String>) -> String {
        let values = self.parse_input(input);
        let mut previous_sum: i64 = values[..3].iter().sum();
        let mut current_sum: i64 = previous_sum;
        let mut result: i64 = 0;
        for i in 3..values.len() {
            previous_sum = current_sum;
            current_sum += values[i];
            current_sum -= values[i - 3];
            if current_sum > previous_sum {
                result += 1;
            }
        }
        result.to_string()
    }
}

impl Day1 {
    fn parse_input(&self, input: Vec<String>) -> Vec<i64> {
        input.iter().map(|str| str.parse().unwrap()).collect()
    }
}
