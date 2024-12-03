use std::env;

use days::day1_historian_hysteria::{day1_2_historian_hysteria, day1_historian_hysteria};

mod days;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        println!("Day arg: {}", args[1]);
        if args[1] == "day" {
            println!("Running code from advent of code day {}", args[1]);
            match args[2].as_str() {
                "1" => day1_historian_hysteria(),
                "1.2" => day1_2_historian_hysteria(),
                _ => panic!("Not a valid date"),
            }
        }
    } else {
        println!("No extra parameters detected.");
    }
}
