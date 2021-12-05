use super::utils::Day;

pub struct Day3;
impl Day for Day3 {
    fn part_1(&self, input: Vec<String>) -> String {
        let counts_of_1 = self.get_counts_of_1(input);
        let gamma_rate = counts_of_1
            .iter()
            .map(|&x| if x > 500 { '1' } else { '0' })
            .collect::<String>();
        let epsilon_rate = counts_of_1
            .iter()
            .map(|&x| if x > 500 { '0' } else { '1' })
            .collect::<String>();
        (i64::from_str_radix(&gamma_rate, 2).unwrap()
            * i64::from_str_radix(&epsilon_rate, 2).unwrap())
        .to_string()
    }

    fn part_2(&self, input: Vec<String>) -> String {
        let counts_of_1 = self.get_counts_of_1(input.clone());
        let oxygen_generator_rating = self.filter_numbers_from_input(
            input.clone(),
            counts_of_1.clone(),
            Commonness::MostCommon,
        );
        let co2_scrubber_rating = self.filter_numbers_from_input(
            input.clone(),
            counts_of_1.clone(),
            Commonness::LeastCommon,
        );
        (i64::from_str_radix(&oxygen_generator_rating, 2).unwrap()
            * i64::from_str_radix(&co2_scrubber_rating, 2).unwrap())
        .to_string()
    }
}

impl Day3 {
    fn get_counts_of_1(&self, input: Vec<String>) -> Vec<i32> {
        let mut sums = vec![0; input.get(0).unwrap().len()];
        for line in input {
            let chars = line.chars().collect::<Vec<char>>();
            chars.iter().enumerate().for_each(|(i, c)| {
                match c {
                    '1' => *sums.get_mut(i).unwrap() += 1,
                    '0' => (),
                    _ => panic!("Invalid input"),
                };
            });
        }
        sums
    }

    fn filter_numbers_from_input(
        &self,
        input: Vec<String>,
        counts_of_1: Vec<i32>,
        commonness: Commonness,
    ) -> String {
        let max_number_of_iterations = counts_of_1.len();
        let mut filtered_inputs = input;
        let mut iterating_counts_of_1 = counts_of_1;
        for i in 0..max_number_of_iterations {
            let len = filtered_inputs.len() as i32;
            let count_to_check_against = len / 2 + len % 2;
            match commonness {
                Commonness::MostCommon => {
                    if iterating_counts_of_1.get(i).unwrap() >= &count_to_check_against {
                        filtered_inputs.retain(|input| &input[i..i + 1] == "1")
                    } else {
                        filtered_inputs.retain(|input| &input[i..i + 1] == "0")
                    }
                }
                Commonness::LeastCommon => {
                    if iterating_counts_of_1.get(i).unwrap() >= &count_to_check_against {
                        filtered_inputs.retain(|input| &input[i..i + 1] == "0")
                    } else {
                        filtered_inputs.retain(|input| &input[i..i + 1] == "1")
                    }
                }
            }
            iterating_counts_of_1 = self.get_counts_of_1(filtered_inputs.clone());
            if filtered_inputs.len() == 1 {
                return filtered_inputs.get(0).unwrap().clone();
            }
        }
        filtered_inputs.get(0).unwrap().clone()
    }
}

#[derive(Debug)]
enum Commonness {
    MostCommon,
    LeastCommon,
}
