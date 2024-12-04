use std::{fs::File, io::Read, ops::Mul, time::Instant};

pub(crate) fn day3_mull_it_over() {
    println!("Running day3");

    let timer = Instant::now();

    let mut file = match File::open("./src/inputs/day3.txt") {
        Ok(day1) => day1,
        Err(e) => {
            panic!("Could not open day1 files, error: {e}");
        }
    };

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Could not read file");

    let mut result = 0;

    let multiply = get_muls_from_str(&content);

    // println!("Multiplys: {:?}", multiply);

    for mul in multiply {
        result += mul.mult()
    }

    println!("Answer: {}, elapsed: {:?}", result, timer.elapsed());
}

#[derive(Debug)]
struct Multiply {
    first: usize,
    second: usize,
}

impl Multiply {
    fn mult(&self) -> usize {
        self.first.mul(self.second)
    }
}

fn get_muls_from_str(input: &str) -> Vec<Multiply> {
    let mut output = vec![];

    let mut is_mul = String::new();

    let mut getting_first = true;

    let mut first = String::new();
    let mut second = String::new();

    for char in input.chars() {
        if char == 'm' {
            is_mul.push(char);
            continue;
        }

        if is_mul.contains("m") && char == 'u' {
            is_mul.push(char);
            continue;
        }

        if is_mul.contains("mu") && char == 'l' {
            is_mul.push(char);
            continue;
        }

        if is_mul.contains("mul") && char == '(' {
            is_mul.push(char);
            continue;
        }

        if is_mul.contains("mul(") && char.is_ascii_digit() {
            if getting_first {
                first.push(char);
            } else {
                second.push(char);
            }
        } else if is_mul.contains("mul(") && char == ',' {
            getting_first = false
        } else if is_mul.contains("mul(") && char == ')' {
            output.push(Multiply {
                first: first.parse().expect("Could not parse first"),
                second: second.parse().expect("Could not parse first"),
            });

            getting_first = true;
            first.clear();
            second.clear();
            is_mul.clear();
        } else {
            getting_first = true;
            first.clear();
            second.clear();
            is_mul.clear();
        }
    }

    output
}

pub(crate) fn day3_2_mull_it_over() {
    println!("Running day3, do / don't");

    let timer = Instant::now();

    let mut file = match File::open("./src/inputs/day3.2.txt") {
        Ok(day1) => day1,
        Err(e) => {
            panic!("Could not open day1 files, error: {e}");
        }
    };

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Could not read file");

    let mut result = 0;

    let multiply = get_specific_muls_from_str(&content);

    // println!("Multiplys: {:?}", multiply);

    for mul in multiply {
        result += mul.mult()
    }

    println!("Answer: {}, elapsed: {:?}", result, timer.elapsed());
}

fn get_specific_muls_from_str(input: &str) -> Vec<Multiply> {
    let mut output = vec![];

    let mut is_mul = String::new();

    // do()
    // don't()
    let mut is_do = true;

    // Find do or dont while finding mulls.
    let mut is_do_dont = String::new();

    let mut getting_first = true;

    let mut first = String::new();
    let mut second = String::new();

    for char in input.chars() {
        if char == 'd' {
            is_do_dont.push(char);
            continue;
        }

        if is_do_dont.contains("d") && char == 'o' {
            is_do_dont.push(char);
            continue;
        }

        // do
        if is_do_dont.contains("do") && char == '(' {
            is_do_dont.push(char);
            continue;
        }

        if is_do_dont.contains("do(") && char == ')' {
            is_do_dont.push(char);

            if is_do_dont.contains("do()") {
                is_do = true;
                is_do_dont.clear();
            }
        }

        // don't()
        if is_do_dont.contains("do") && char == 'n' {
            is_do_dont.push(char);
            continue;
        }

        // don't()
        if is_do_dont.contains("don") && char == '\'' {
            is_do_dont.push(char);
            continue;
        }

        // don't()
        if is_do_dont.contains("don'") && char == 't' {
            is_do_dont.push(char);
            continue;
        }

        // don't()
        if is_do_dont.contains("don't") && char == '(' {
            is_do_dont.push(char);
            continue;
        }

        // don't()
        if is_do_dont.contains("don't(") && char == ')' {
            is_do_dont.push(char);

            if is_do_dont.contains("don't()") {
                is_do = false;
                is_do_dont.clear();
            }
        }

        if !is_do {
            continue;
        }

        if char == 'm' {
            is_mul.push(char);
            continue;
        }

        if is_mul.contains("m") && char == 'u' {
            is_mul.push(char);
            continue;
        }

        if is_mul.contains("mu") && char == 'l' {
            is_mul.push(char);
            continue;
        }

        if is_mul.contains("mul") && char == '(' {
            is_mul.push(char);
            continue;
        }

        if is_mul.contains("mul(") && char.is_ascii_digit() {
            if getting_first {
                first.push(char);
            } else {
                second.push(char);
            }
        } else if is_mul.contains("mul(") && char == ',' {
            getting_first = false
        } else if is_mul.contains("mul(") && char == ')' {
            output.push(Multiply {
                first: first.parse().expect("Could not parse first"),
                second: second.parse().expect("Could not parse first"),
            });

            getting_first = true;
            first.clear();
            second.clear();
            is_mul.clear();
        } else {
            getting_first = true;
            first.clear();
            second.clear();
            is_mul.clear();
        }
    }

    output
}
