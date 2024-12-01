use std::env;

use days::day1_historian_hysteria::day1_historian_hysteria;

mod days;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        if args[0] == "day" {
            match args[1].as_str() {
                "1" => day1_historian_hysteria(),
                _ => panic!("Not a valid date"),
            }
        }
    } else {
        println!("No extra parameters detected.");
    }
}
