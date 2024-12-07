#![allow(clippy::needless_return)]

use std::env;

use days::{
    day1_historian_hysteria::{day1_2_historian_hysteria, day1_historian_hysteria},
    day2_red_nosed_reports::{day2_2_red_nosed_reports, day2_red_nosed_reports},
    day3_mull_it_over::{day3_2_mull_it_over, day3_mull_it_over},
    day4_ceres_search::{day4_2_ceres_search, day4_ceres_search},
    day5_print_queue::day5_print_queue,
};

mod days;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        println!("Day arg: {}", args[1]);
        if args[1] == "day" {
            println!("Running code from advent of code day {}", args[1]);
            match args[2].as_str() {
                "1" => day1_historian_hysteria(),
                "1.2" => day1_2_historian_hysteria(),
                "2" => day2_red_nosed_reports(),
                "2.2" => day2_2_red_nosed_reports(),
                "3" => day3_mull_it_over(),
                "3.2" => day3_2_mull_it_over(),
                "4" => day4_ceres_search(),
                "4.2" => day4_2_ceres_search(),
                "5" => day5_print_queue(),
                _ => panic!("Not a valid day"),
            }
        }
    } else {
        println!("No extra parameters detected.");
    }
}
