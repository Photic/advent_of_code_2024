use std::collections::HashMap;

use crate::utils::get_utility::{end_day, start_day};

pub(crate) fn day11_plutonian_pebbles() {
    println!("Running day11 Plutonian Pebblest");

    let (content, timer) = start_day("./src/inputs/day11.txt");

    let blinks = 25;
    let number_array = array_from_content(&content);

    let stone_count = play_with_stones(blinks, number_array);

    end_day(&stone_count, &timer);
}

fn play_with_stones(blinks: usize, number_array: Vec<u64>) -> usize {
    let mut unique_stones: HashMap<u64, usize> = HashMap::with_capacity(5000);

    for engraving in number_array {
        *unique_stones.entry(engraving).or_insert(0) += 1;
    }

    for _ in 0..blinks {
        let stonework = unique_stones.clone();
        for (stone, count) in stonework.iter() {
            if *count == 0 {
                continue;
            }

            if *stone == 0 {
                *unique_stones.entry(1).or_insert(0) += *count;
                *unique_stones.entry(0).or_insert(0) -= *count;
            } else {
                if stone.ilog10() % 2 == 1 {
                    let digits_count = stone.ilog10() + 1;
                    let power = 10_u64.pow(digits_count / 2);

                    *unique_stones.entry(stone / power).or_insert(0) += *count;
                    *unique_stones.entry(stone % power).or_insert(0) += *count;
                    *unique_stones.entry(*stone).or_insert(0) -= *count;
                } else {
                    let new_value = stone * 2024;
                    *unique_stones.entry(new_value).or_insert(0) += *count;
                    *unique_stones.entry(*stone).or_insert(0) -= *count;
                }
            }
        }
    }

    unique_stones.values().sum()
}

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

    let stone_count = play_with_stones(blinks, number_array);

    end_day(&(stone_count as usize), &timer);
}
