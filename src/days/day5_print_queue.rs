use crate::utils::get_utility::start_day;

pub(crate) fn day5_print_queue() {
    println!("Running day5 Print Queue");

    let (content, timer) = start_day("./src/inputs/day5_test.txt");

    let (order, arrays) = format_files(&content);
}

fn format_files(input: &str) -> (Vec<(usize, usize)>, Vec<Vec<usize>>) {
    let mut order: Vec<(usize, usize)> = vec![];
    let mut arrays: Vec<Vec<usize>> = vec![];

    let mut left_digit = String::with_capacity(2);
    let mut right_digit = String::with_capacity(2);

    let mut current_array: Vec<usize> = vec![];

    let mut middle = false;

    for char in input.chars() {
        if char.is_ascii_digit() && !middle {
            left_digit.push(char);
        } else if char.is_ascii_digit() && middle {
            right_digit.push(char);
        } else if char == '|' {
            middle = true
        } else if char == '\n' {
            if !current_array.is_empty() {
                current_array.push(left_digit.parse().expect("Could not parse int"));
                left_digit.clear();
                arrays.push(current_array.clone());
                current_array.clear();
            }

            middle = false
        } else if char == ',' {
            current_array.push(left_digit.parse().expect("Could not parse int"));
            left_digit.clear();
        }

        if left_digit.len() == 2 && right_digit.len() == 2 {
            order.push((
                left_digit.parse().expect("Could not parse int"),
                right_digit.parse().expect("Could not parse int"),
            ));
            left_digit.clear();
            right_digit.clear();
        }
    }

    arrays.push(current_array);

    println!("order: {:?},\narrays: {:?}", order, arrays);

    (order, arrays)
}
