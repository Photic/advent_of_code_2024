use crate::utils::get_utility::{end_day, start_day, Cord};

pub(crate) fn day5_print_queue() {
    println!("Running day5 Print Queue");

    let (content, timer) = start_day("./src/inputs/day5.txt");

    let (ordering_rules, pages_to_produce) = format_files(&content);

    let mut pages_that_fits = vec![];

    produce_pages_that_fits(ordering_rules, pages_to_produce, &mut pages_that_fits);

    // println!("Pages that fits: {:?}", pages_that_fits);

    let mut result = 0;

    for pages in pages_that_fits {
        let index = pages.len() / 2;
        if let Some(page) = pages.get(index) {
            result += page
        }
    }

    end_day(&result, &timer);
}

fn produce_pages_that_fits(
    ordering_rules: Vec<Cord>,
    pages_to_produce: Vec<Vec<usize>>,
    pages_that_fits: &mut Vec<Vec<usize>>,
) {
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
            pages_that_fits.push(pages);
        }
    }
}

pub(crate) fn day5_2_print_queue() {
    println!("Running day5.2 Print Queue does not fit");

    let (content, timer) = start_day("./src/inputs/day5.txt");

    let (ordering_rules, pages_to_produce) = format_files(&content);

    let mut pages_that_fits = vec![];

    produce_pages_that_fits(
        ordering_rules.clone(),
        pages_to_produce.clone(),
        &mut pages_that_fits,
    );

    let pages_that_dont_fit: Vec<Vec<usize>> = pages_to_produce
        .clone()
        .into_iter()
        .filter(|page| !pages_that_fits.contains(page))
        .collect();

    // println!("Pages that does not fit: {:?}", pages_that_dont_fit);

    let mut pages_that_now_fits: Vec<Vec<usize>> = vec![];

    for mut pages in pages_that_dont_fit.clone() {
        let mut pages_now_fit: Vec<Vec<usize>> = vec![];

        while pages_now_fit.len() != 1 {
            for rule in ordering_rules.clone() {
                if let (Some(x_p), Some(y_p)) = (
                    pages.iter().position(|&x| x == rule.x),
                    pages.iter().position(|&y| y == rule.y),
                ) {
                    if x_p > y_p {
                        pages.swap(y_p, x_p);

                        produce_pages_that_fits(
                            ordering_rules.clone(),
                            vec![pages.clone()],
                            &mut pages_now_fit,
                        );
                    }
                }
            }

            // println!("Pages {:?}", pages);
        }

        pages_that_now_fits.push(pages.clone());
        // println!("Swapped: {:?}", pages);
    }

    // println!("Pages that now fit: {:?}", pages_that_now_fits);

    let mut result = 0;

    for pages in pages_that_now_fits {
        if let Some(page) = pages.get(pages.len() / 2) {
            result += page;
        }
    }

    end_day(&result, &timer);
}

fn format_files(input: &str) -> (Vec<Cord>, Vec<Vec<usize>>) {
    let mut ordering_rules: Vec<Cord> = vec![];
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
            ordering_rules.push(Cord {
                x: left_digit.parse().expect("Could not parse int"),
                y: right_digit.parse().expect("Could not parse int"),
            });
            left_digit.clear();
            right_digit.clear();
        }
    }

    current_array.push(left_digit.parse().expect("Could not parse int"));
    pages_to_produce.push(current_array);

    // println!(
    //     "order: {:?},\npages_to_produce: {:?}",
    //     ordering_rules, pages_to_produce
    // );

    (ordering_rules, pages_to_produce)
}
