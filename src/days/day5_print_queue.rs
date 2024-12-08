use crate::utils::get_utility::{end_day, start_day};

pub(crate) fn day5_print_queue() {
    println!("Running day5 Print Queue");

    let (content, timer) = start_day("./src/inputs/day5.txt");

    let (ordering_rules, pages_to_produce) = format_files(&content);

    let mut pages_that_fits = vec![];

    for pages in pages_to_produce {
        let mut ordering_fits = true;

        for rule in ordering_rules.clone() {
            if let (Some(x_p), Some(y_p)) = (
                pages.iter().position(|&x| x == rule.x),
                pages.iter().position(|&y| y == rule.y),
            ) {
                // println!("x_p: {}, y_p: {}", x_p, y_p);
                if x_p > y_p {
                    ordering_fits = false;
                    break;
                }
            }
        }

        if ordering_fits {
            pages_that_fits.push(pages.clone());
        }
    }

    // println!("Pages that fits: {:?}", pages_that_fits);

    let mut result = 0;

    for pages in pages_that_fits {
        let index = pages.len() / 2;
        if let Some(page) = pages.get(index) {
            result += page
        }
    }

    end_day(&result.to_string(), &timer);
}

#[derive(Debug, Clone, PartialEq)]
struct XY {
    x: usize,
    y: usize,
}

fn format_files(input: &str) -> (Vec<XY>, Vec<Vec<usize>>) {
    let mut ordering_rules: Vec<XY> = vec![];
    let mut pages_to_produce: Vec<Vec<usize>> = vec![];

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
                pages_to_produce.push(current_array.clone());
                current_array.clear();
            }

            middle = false
        } else if char == ',' {
            current_array.push(left_digit.parse().expect("Could not parse int"));
            left_digit.clear();
        }

        if left_digit.len() == 2 && right_digit.len() == 2 {
            ordering_rules.push(XY {
                x: left_digit.parse().expect("Could not parse int"),
                y: right_digit.parse().expect("Could not parse int"),
            });
            left_digit.clear();
            right_digit.clear();
        }
    }

    pages_to_produce.push(current_array);

    // println!(
    //     "order: {:?},\npages_to_produce: {:?}",
    //     ordering_rules, pages_to_produce
    // );

    (ordering_rules, pages_to_produce)
}
