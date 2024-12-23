use crate::utils::get_utility::{end_day, start_day, ResultType};

pub(crate) fn day4_ceres_search() {
    println!("Running day4 XMAS");

    let (content, timer) = start_day("./src/inputs/day4.txt");

    let arrays = string_to_2d_array(&content);

    // println!("Array: {:?}", arrays);

    let result = find_xmas(&arrays);

    end_day(&ResultType::Num(result), &timer);
}

pub(crate) fn day4_2_ceres_search() {
    println!("Running day4 X-MAS");

    let (content, timer) = start_day("./src/inputs/day4.2.txt");

    let arrays = string_to_2d_array(&content);

    // println!("Array: {:?}", arrays);

    let result = find_x_mas(&arrays);

    end_day(&ResultType::Num(result), &timer);
}

fn find_x_mas(arrays: &[Vec<char>]) -> usize {
    let mut occurense = 0;

    let find_xmas = "MAS";
    let find_xmas_reversed: String = find_xmas.chars().rev().collect();

    for (y, array) in arrays.iter().enumerate().skip(1) {
        for (x, &char) in array.iter().enumerate() {
            if char == 'A' {
                let mut current_word_right_diagonal = String::new();
                let mut current_word_left_diagonal = String::new();

                if y > 0 && y < arrays.len() - 1 {
                    if let Some(array_exists_above) = arrays.get(y - 1) {
                        if let Some(array_exists_below) = arrays.get(y + 1) {
                            if x > 0
                                && x < array_exists_above.len() - 1
                                && x < array_exists_below.len() - 1
                            {
                                current_word_right_diagonal = format!(
                                    "{}A{}",
                                    array_exists_above.get(x - 1).unwrap_or(&'e'),
                                    array_exists_below.get(x + 1).unwrap_or(&'e')
                                );

                                current_word_left_diagonal = format!(
                                    "{}A{}",
                                    array_exists_above.get(x + 1).unwrap_or(&'e'),
                                    array_exists_below.get(x - 1).unwrap_or(&'e')
                                );
                            }
                        }
                    }
                }

                if (current_word_left_diagonal == find_xmas
                    || current_word_left_diagonal == find_xmas_reversed)
                    && (current_word_right_diagonal == find_xmas
                        || current_word_right_diagonal == find_xmas_reversed)
                {
                    occurense += 1;
                }
            }
        }
    }

    occurense
}

fn find_xmas(arrays: &[Vec<char>]) -> usize {
    let mut occurense = 0;

    let find_xmas = "XMAS";
    let find_xmas_reversed: String = find_xmas.chars().rev().collect();

    let directions = [
        (1, 0),   // Right
        (-1, 0),  // Left
        (0, 1),   // Down
        (0, -1),  // Up
        (1, 1),   // Down-Right
        (-1, -1), // Up-Left
        (1, -1),  // Up-Right
        (-1, 1),  // Down-Left
    ];

    for (y, array) in arrays.iter().enumerate() {
        for (x, &char) in array.iter().enumerate() {
            if char == 'X' {
                for &(dx, dy) in &directions {
                    let mut current_word = String::new();
                    current_word.push(char);

                    for i in 1..4 {
                        let nx = x as isize + i * dx;
                        let ny = y as isize + i * dy;

                        if nx < 0 || ny < 0 {
                            break;
                        }

                        let nx = nx as usize;
                        let ny = ny as usize;

                        if let Some(row) = arrays.get(ny) {
                            if let Some(&next_char) = row.get(nx) {
                                current_word.push(next_char);
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    }

                    if current_word == find_xmas || current_word == find_xmas_reversed {
                        occurense += 1;
                    }
                }
            }
        }
    }

    occurense
}

pub(crate) fn string_to_2d_array(input: &str) -> Vec<Vec<char>> {
    let mut output: Vec<Vec<char>> = vec![];

    let mut current_chars: Vec<char> = vec![];

    for char in input.chars() {
        if char != '\n' {
            current_chars.push(char);
        } else {
            output.push(current_chars.clone());
            current_chars.clear();
        }
    }

    output.push(current_chars);

    output
}
