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
