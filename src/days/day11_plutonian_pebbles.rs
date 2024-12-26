use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::utils::get_utility::{end_day, start_day};

pub(crate) fn day11_plutonian_pebbles() {
    println!("Running day11 Plutonian Pebblest");

    let (content, timer) = start_day("./src/inputs/day11.txt");

    let mut blinks_max = 25;
    let number_array = array_from_content(&content);

    let new_stones = play_with_stones(&mut blinks_max, &number_array);

    end_day(&new_stones.len(), &timer);
}

fn play_with_stones(blinks_max: &mut usize, number_array: &Vec<i64>) -> Vec<i64> {
    handle_stones(blinks_max, number_array)
}

fn handle_stones(blinks_max: &mut usize, number_array: &Vec<i64>) -> Vec<i64> {
    if *blinks_max == 0 {
        return number_array.clone();
    }

    let new_numbers: Vec<i64> = number_array
        .par_iter()
        .flat_map(|&number| {
            if count_digits(&number) % 2 == 0 {
                split_number(&number).into_iter().collect::<Vec<i64>>()
            } else if number == 0 {
                vec![1]
            } else {
                vec![number * 2024]
            }
        })
        .collect();

    *blinks_max -= 1;
    return handle_stones(blinks_max, &new_numbers);

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
}

fn split_number(number: &i64) -> Vec<i64> {
    let number_string = number.to_string();
    let split_number = number_string.split_at(number_string.len() / 2);

    vec![
        split_number
            .0
            .parse()
            .expect("Could not parse split_number"),
        split_number
            .1
            .parse()
            .expect("Could not parse split_number"),
    ]
}

fn count_digits(number: &i64) -> usize {
    number.abs().to_string().chars().count()
}

fn array_from_content(input: &str) -> Vec<i64> {
    let mut output = vec![];

    let mut current_number = String::new();

    for char in input.chars() {
        if char == ' ' {
            output.push(
                current_number
                    .clone()
                    .parse::<i64>()
                    .expect("Could not parse number"),
            );
            current_number.clear();
        } else {
            current_number.push(char);
        }
    }

    output.push(
        current_number
            .clone()
            .parse::<i64>()
            .expect("Could not parse number"),
    );

    output
}

pub(crate) fn day11_2_plutonian_pebbles() {
    println!("Running day11.2 Plutonian Pebblest");

    let (content, timer) = start_day("./src/inputs/day11_test.txt");

    let mut blinks_max = 75;
    let number_array = array_from_content(&content);

    let new_stones = play_with_stones(&mut blinks_max, &number_array);

    end_day(&new_stones.len(), &timer);
}
