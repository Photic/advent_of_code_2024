use std::{fmt, fs::File, io::Read, time::Instant};

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

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy, Ord, PartialOrd, Default)]
pub(crate) struct Cord {
    pub(crate) x: usize,
    pub(crate) y: usize,
}

#[derive(PartialEq, Clone, Debug, Eq, Hash)]
pub(crate) enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub(crate) fn new_position_in_direction(position: &Cord, direction: &Direction) -> Option<Cord> {
    match direction {
        Direction::Up => Some(Cord {
            x: position.x.checked_sub(1)?,
            y: position.y,
        }),
        Direction::Down => Some(Cord {
            x: position.x.checked_add(1)?,
            y: position.y,
        }),
        Direction::Left => Some(Cord {
            x: position.x,
            y: position.y.checked_sub(1)?,
        }),
        Direction::Right => Some(Cord {
            x: position.x,
            y: position.y.checked_add(1)?,
        }),
    }
}

pub(crate) const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

#[derive(Clone, Debug)]
pub(crate) struct Node {
    pub(crate) height: usize,
    pub(crate) cord: Cord,
    pub(crate) visited: bool,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.height)
    }
}
