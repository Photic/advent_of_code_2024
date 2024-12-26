use std::collections::HashMap;

use crate::utils::get_utility::{end_day, start_day};

pub(crate) fn day11_plutonian_pebbles() {
    println!("Running day11 Plutonian Pebblest");

    let (content, timer) = start_day("./src/inputs/day11.txt");

    let blinks = 25;
    let number_array = array_from_content(&content);

    // let new_stones = play_with_stones(&mut blinks_max, &number_array);

    let stone_count = play_with_stones(&number_array, blinks);

    end_day(&(stone_count as usize), &timer);
}

// Borrowed from https://github.com/maneatingape/advent-of-code-rust/blob/main/src/year2024/day11.rs
// Want to learn this code better later.
fn play_with_stones(input: &[u64], blinks: usize) -> u64 {
    let mut stones = Vec::with_capacity(5000);
    let mut indices = HashMap::with_capacity(5000);

    let mut todo = Vec::new();
    let mut current = Vec::new();

    for &number in input {
        if let Some(&index) = indices.get(&number) {
            current[index] += 1;
        } else {
            indices.insert(number, indices.len());
            todo.push(number);
            current.push(1);
        }
    }

    for _ in 0..blinks {
        let numbers = todo;
        todo = Vec::with_capacity(200);

        let mut index_of = |number| {
            let size = indices.len();
            *indices.entry(number).or_insert_with(|| {
                todo.push(number);
                size
            })
        };

        for number in numbers {
            let (first, second) = if number == 0 {
                (index_of(1), usize::MAX)
            } else {
                let digits = number.ilog10() + 1;
                if digits % 2 == 0 {
                    let power = 10_u64.pow(digits / 2);
                    (index_of(number / power), index_of(number % power))
                } else {
                    (index_of(number * 2024), usize::MAX)
                }
            };

            stones.push((first, second));
        }

        let mut next = vec![0; indices.len()];

        for (&(first, second), amount) in stones.iter().zip(current) {
            next[first] += amount;
            if second != usize::MAX {
                next[second] += amount;
            }
        }

        current = next;
    }

    println!("Stones: {:?}", stones);

    current.iter().sum()
}

// fn play_with_stones(blinks_max: &mut usize, number_array: &Vec<i64>) -> Vec<i64> {
//     handle_stones(blinks_max, number_array)
// }

// fn handle_stones(blinks_max: &mut usize, number_array: &Vec<i64>) -> Vec<i64> {
//     if *blinks_max == 0 {
//         return number_array.clone();
//     }

// let new_numbers: Vec<i64> = number_array
//     .par_iter()
//     .flat_map(|&number| {
//         if count_digits(&number) % 2 == 0 {
//             split_number(&number).into_iter().collect::<Vec<i64>>()
//         } else if number == 0 {
//             vec![1]
//         } else {
//             vec![number * 2024]
//         }
//     })
//     .collect();

// *blinks_max -= 1;
// return handle_stones(blinks_max, &new_numbers);

// let mut new_numbers = vec![];

// for number in number_array {
//     if count_digits(&number) % 2 == 0 {
//         for number in split_number(&number) {
//             new_numbers.push(number);
//         }
//     } else if number == 0 {
//         new_numbers.push(1);
//     } else {
//         new_numbers.push(number * 2024);
//     }
// }

// *blinks += 1;

// println!("Number Array: {:?}", new_numbers);
// return handle_stones(blinks, blinks_max, new_numbers);
// }

// fn split_number(number: &i64) -> Vec<i64> {
//     let number_string = number.to_string();
//     let split_number = number_string.split_at(number_string.len() / 2);

//     vec![
//         split_number
//             .0
//             .parse()
//             .expect("Could not parse split_number"),
//         split_number
//             .1
//             .parse()
//             .expect("Could not parse split_number"),
//     ]
// }

// fn count_digits(number: &i64) -> usize {
//     number.abs().to_string().chars().count()
// }

fn array_from_content(input: &str) -> Vec<u64> {
    let mut output = vec![];

    for number in input.trim().split(" ") {
        output.push(number.parse::<u64>().expect("Could not parse number"));
    }

    output
}

pub(crate) fn day11_2_plutonian_pebbles() {
    println!("Running day11.2 Plutonian Pebblest");

    let (content, timer) = start_day("./src/inputs/day11.txt");

    let blinks = 75;
    let number_array = array_from_content(&content);

    // let new_stones = play_with_stones(&mut blinks_max, &number_array);

    // end_day(&new_stones.len(), &timer);

    let stone_count = play_with_stones(&number_array, blinks);

    end_day(&(stone_count as usize), &timer);
}
