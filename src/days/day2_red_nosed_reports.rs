use std::{fs::File, io::Read, time::Instant};

pub(crate) fn day2_red_nosed_reports() {
    println!("Running day2");

    let timer = Instant::now();

    let mut file = match File::open("./src/inputs/day2.txt") {
        Ok(day1) => day1,
        Err(e) => {
            panic!("Could not open day1 files, error: {e}");
        }
    };

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Could not read file");

    // println!("Content:\n{}", content);

    let number_arrays = convert_string_to_arrays(&content);

    // println!("Number_arrays: {:?}", number_arrays);

    let mut safe_reports = 0;

    get_safe_answers(&mut safe_reports, number_arrays);

    println!("Answer: {}, elapsed: {:?}", safe_reports, timer.elapsed())
}

fn get_safe_answers(safe_reports: &mut usize, number_arrays: Vec<Vec<i64>>) {
    for array in number_arrays {
        if is_safe_report(&array) {
            *safe_reports += 1;
        }
    }
}

fn is_safe_report(array: &Vec<i64>) -> bool {
    let mut last_number: i64 = 0;

    let mut report_safe = true;

    let mut increase = false;
    let mut decrease = false;

    for (index, number) in array.iter().enumerate() {
        let number = number.to_owned();
        if index == 0 {
            last_number = number;
            continue;
        }

        if last_number.abs_diff(number) > 3 || last_number == number {
            report_safe = false;
        }

        if last_number > number && !decrease {
            decrease = true;
        }

        if last_number < number && !increase {
            increase = true;
        }

        last_number = number;
    }

    if (increase || decrease) && (!increase || !decrease) && report_safe {
        // println!(
        //     "Increase: {increase}, Decrease: {decrease}, and array: {:?}",
        //     array
        // );
        return true;
    }

    return false;
}

pub(crate) fn day2_2_red_nosed_reports() {
    println!("Running day2.2 Problem Dampener");

    let timer = Instant::now();

    let mut file = match File::open("./src/inputs/day2.txt") {
        Ok(day1) => day1,
        Err(e) => {
            panic!("Could not open day1 files, error: {e}");
        }
    };

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Could not read file");

    // println!("Content:\n{}", content);

    let number_arrays = convert_string_to_arrays(&content);

    // println!("Number_arrays: {:?}", number_arrays);

    let mut safe_reports = 0;

    for array in number_arrays {
        if is_safe_report(&array) {
            safe_reports += 1;
            continue;
        }

        for (index, _) in array.iter().enumerate() {
            if is_safe_report(
                &array
                    .iter()
                    .enumerate()
                    .filter(|&(i, _)| i != index)
                    .map(|(_, v)| v.clone())
                    .collect(),
            ) {
                safe_reports += 1;
                break;
            }
        }
    }

    println!("Answer: {}, elapsed: {:?}", safe_reports, timer.elapsed())
}

fn convert_string_to_arrays(string: &str) -> Vec<Vec<i64>> {
    let mut output_vectors: Vec<Vec<i64>> = vec![];

    let mut number: String = String::new();

    let mut current_vector: Vec<i64> = vec![];

    for char in string.chars() {
        if let Some(_) = char.to_digit(10) {
            number.push_str(&char.to_string());
        } else if char == '\n' {
            current_vector.push(number.parse().expect("Could not parse number"));
            number.clear();
            output_vectors.push(current_vector.clone());
            current_vector.clear();
        } else {
            current_vector.push(number.parse().expect("Could not parse number"));
            number.clear();
        }
    }

    if !current_vector.is_empty() && !number.is_empty() {
        current_vector.push(number.parse().expect("Could not parse number"));
        output_vectors.push(current_vector);
    }

    output_vectors
}
