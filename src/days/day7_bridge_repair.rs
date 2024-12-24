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
            let iterations_max = 3_usize.pow((equation.1.len() - 1) as u32);
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
                let mut add_multiply_combine = vec![];

                for _ in 0..test_len {
                    let rand_choice = rng.gen_range(0..3);
                    match rand_choice {
                        0 => add_multiply_combine.push('+'),
                        1 => add_multiply_combine.push('*'),
                        _ => add_multiply_combine.push('>'),
                    }
                }

                if !equations_tested.contains(&add_multiply_combine) {
                    equations_tested.insert(add_multiply_combine.clone());

                    let mut equation_result = equation.1[0];

                    for (index, number) in equation.1.iter().enumerate().skip(1) {
                        let operator = add_multiply_combine
                            .get(index - 1)
                            .expect("Could not get operator");

                        if *operator == '+' {
                            equation_result += number;
                        } else if *operator == '*' {
                            equation_result *= number;
                        } else if *operator == '>' {
                            equation_result = format!("{}{}", equation_result, number)
                                .parse()
                                .expect("Could not parse number");
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
