use std::{fs::File, io::Read};

pub(crate) fn day1_historian_hysteria() {
    println!("Running day1");

    let mut file = match File::open("./src/inputs/day1.txt") {
        Ok(day1) => day1,
        Err(e) => {
            panic!("Could not open day1 files, error: {e}");
        }
    };

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Could not read file");

    // println!("Content:\n{}", content);

    let (mut left_numbers, mut right_numbers) = convert_string_to_double_array(&content);

    left_numbers.sort();
    right_numbers.sort();

    // println!(
    //     "Number left: {:?},\n right: {:?}",
    //     left_numbers, right_numbers
    // );

    let mut output_result: i64 = 0;

    for (index, number) in left_numbers.iter().enumerate() {
        output_result += (number - right_numbers[index]).abs();
    }

    println!("Answer: {}", output_result);
}

pub(crate) fn day1_2_historian_hysteria() {
    println!("Running day1");

    let mut file = match File::open("./src/inputs/day1.txt") {
        Ok(day1) => day1,
        Err(e) => {
            panic!("Could not open day1 files, error: {e}");
        }
    };

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Could not read file");

    // println!("Content:\n{}", content);

    let (left_numbers, right_numbers) = convert_string_to_double_array(&content);

    let mut similarity_score = 0;

    for number_left in left_numbers {
        let mut number_appears = 0;

        for number_right in right_numbers.clone() {
            if number_left == number_right {
                number_appears += 1;
            }
        }

        similarity_score += number_left * number_appears
    }

    println!("Answer: {}", similarity_score);
}

fn convert_string_to_double_array(string: &str) -> (Vec<i64>, Vec<i64>) {
    let mut left_numbers: Vec<i64> = vec![];
    let mut right_number: Vec<i64> = vec![];

    let mut first_number: String = String::new();
    let mut second_number: String = String::new();

    let mut last_char_was_space: bool = false;

    for char in string.chars() {
        if let Some(_) = char.to_digit(10) {
            if last_char_was_space {
                second_number.push_str(&char.to_string());
            } else {
                first_number.push_str(&char.to_string());
            }
        } else if char == '\n' {
            last_char_was_space = false;

            left_numbers.push(first_number.parse().expect("Should be a number"));
            right_number.push(second_number.parse().expect("Should be a number"));

            first_number.clear();
            second_number.clear();
        } else {
            last_char_was_space = true;
        }
    }

    // Push the last pair of numbers if any
    if !first_number.is_empty() && !second_number.is_empty() {
        left_numbers.push(first_number.parse().expect("Should be a number"));
        right_number.push(second_number.parse().expect("Should be a number"));
    }

    (left_numbers, right_number)
}
