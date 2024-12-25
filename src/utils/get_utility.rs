use std::{fs::File, io::Read, time::Instant};

pub(crate) fn start_day(path: &str) -> (String, Instant) {
    let timer = Instant::now();

    let mut file = match File::open(path) {
        Ok(day1) => day1,
        Err(e) => {
            panic!("Could not open dayX file, error: {e}");
        }
    };

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Could not read file");

    (content, timer)
}

pub(crate) fn end_day(result: &usize, timer: &Instant) {
    println!("Answer: {}, elapsed: {:?}", result, timer.elapsed())
}

// pub(crate) fn end_day(result: &str, timer: &Instant) {
//     println!("Answer: {}, elapsed: {:?}", result, timer.elapsed());
// }

// pub(crate) fn end_day_number(result: &usize, timer: &Instant) {
//     println!("Answer: {}, elapsed: {:?}", result, timer.elapsed());
// }

pub(crate) fn print_2d_array(input: &Vec<Vec<char>>) {
    println!("\n");
    for array in input {
        println!("{:?}\n", array);
    }
}

pub(crate) fn content_to_2d_array(input: &str, border: bool) -> Vec<Vec<char>> {
    let mut output = vec![];

    let mut working_array: Vec<char> = vec![];

    for char in input.chars() {
        if char == '\n' {
            output.push(working_array.clone());
            working_array.clear();
        } else {
            working_array.push(char);
        }
    }

    output.push(working_array);

    if border {
        // Add boundary
        let width = output[0].len();
        let mut bordered_output = vec![vec!['*'; width + 2]];

        for row in output {
            let mut bordered_row = vec!['*'];
            bordered_row.extend(row);
            bordered_row.push('*');
            bordered_output.push(bordered_row);
        }

        bordered_output.push(vec!['*'; width + 2]);

        return bordered_output;
    }

    output
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy, Ord, PartialOrd)]
pub(crate) struct Cords {
    pub(crate) x: usize,
    pub(crate) y: usize,
}
