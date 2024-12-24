use std::collections::HashSet;

use rand::Rng;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::utils::get_utility::{end_day, start_day};

pub(crate) fn day7_bridge_repair() {
    println!("Running day7 Bridge Repair");

    let (content, timer) = start_day("./src/inputs/day7.txt");

    let equation_map = create_equation_map(&content);

    let result: usize = equation_map
        .par_iter()
        .map(|equation| {
            let iterations_max = 2_usize.pow((equation.1.len() - 1) as u32);
            let mut equations_tested: HashSet<Vec<char>> = HashSet::new();
            let mut rng = rand::thread_rng();

            if equation.0 == equation.1.iter().sum() {
                return equation.0;
            }

            if equation.0 == equation.1.iter().fold(1, |acc, &x| acc * x) {
                return equation.0;
            }

            loop {
                let test_len = equation.1.len() - 1;
                let mut add_multiply = vec![];

                for _ in 0..test_len {
                    if rng.gen_bool(0.5) {
                        add_multiply.push('+');
                    } else {
                        add_multiply.push('*');
                    }
                }

                if !equations_tested.contains(&add_multiply) {
                    equations_tested.insert(add_multiply.clone());

                    let mut equation_result = equation.1[0];

                    for (index, number) in equation.1.iter().enumerate().skip(1) {
                        let operator = add_multiply.get(index - 1).expect("Could not get operator");

                        if *operator == '+' {
                            equation_result += number;
                        } else if *operator == '*' {
                            equation_result *= number;
                        }
                    }

                    if equation.0 == equation_result {
                        return equation_result;
                    }
                }

                if equations_tested.len() == iterations_max {
                    return 0;
                }
            }
        })
        .sum();

    end_day(&result, &timer);
}

pub(crate) fn day7_2_bridge_repair() {
    println!("Running day7.2 Bridge Repair");

    let (content, timer) = start_day("./src/inputs/day7.txt");

    let equation_map = create_equation_map(&content);

    let result: usize = equation_map
        .par_iter()
        .map(|equation| {
            let mut equations_tested: HashSet<Vec<char>> = HashSet::new();

            if equation.0 == equation.1.iter().sum() {
                return equation.0;
            }

            if equation.0 == equation.1.iter().product() {
                return equation.0;
            }

            let operators = vec!['+', '*', '>'];
            let test_len = equation.1.len() - 1;

            // DFS
            let generate_combinations = |len: usize| -> Vec<Vec<char>> {
                let mut combinations = vec![];
                let mut stack = vec![(vec![], 0)];

                // Loop until the stack is empty
                while let Some((current, index)) = stack.pop() {
                    if index == len {
                        combinations.push(current);
                    } else {
                        for &op in &operators {
                            let mut new_current = current.clone();
                            new_current.push(op);
                            stack.push((new_current, index + 1));
                        }
                    }
                }

                combinations
            };

            let all_combinations = generate_combinations(test_len);

            for add_multiply_combine in all_combinations {
                if !equations_tested.contains(&add_multiply_combine) {
                    equations_tested.insert(add_multiply_combine.clone());

                    let mut equation_result = equation.1[0];

                    for (index, number) in equation.1.iter().enumerate().skip(1) {
                        let operator = add_multiply_combine
                            .get(index - 1)
                            .expect("Could not get operator");

                        match *operator {
                            '+' => equation_result += number,
                            '*' => equation_result *= number,
                            '>' => {
                                equation_result = format!("{}{}", equation_result, number)
                                    .parse()
                                    .expect("Could not parse number");
                            }
                            _ => unreachable!(),
                        }
                    }

                    if equation.0 == equation_result {
                        return equation_result;
                    }
                }
            }

            return 0;
        })
        .sum();

    end_day(&result, &timer);
}

fn create_equation_map(input: &str) -> Vec<(usize, Vec<usize>)> {
    let mut output: Vec<(usize, Vec<usize>)> = vec![];

    let split_input = input.split('\n');

    for line in split_input {
        let values = line.split(':').into_iter().collect::<Vec<&str>>();

        let test_value: usize = values
            .get(0)
            .unwrap()
            .parse()
            .expect("Could not parse to usize");

        let numbers: Vec<usize> = values
            .get(1)
            .unwrap()
            .trim()
            .split(' ')
            .into_iter()
            .map(|str| str.parse::<usize>().expect("Could not parse int"))
            .collect();

        output.push((test_value, numbers));
    }

    output
}
