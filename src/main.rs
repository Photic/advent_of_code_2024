#![allow(clippy::needless_return)]

use std::env;

use days::{
    day1_historian_hysteria::{day1_2_historian_hysteria, day1_historian_hysteria},
    day2_red_nosed_reports::{day2_2_red_nosed_reports, day2_red_nosed_reports},
    day3_mull_it_over::{day3_2_mull_it_over, day3_mull_it_over},
    day4_ceres_search::{day4_2_ceres_search, day4_ceres_search},
    day5_print_queue::{day5_2_print_queue, day5_print_queue},
    day6_guard_gallivant::{day6_2_guard_gallivant, day6_guard_gallivant},
    day7_bridge_repair::{day7_2_bridge_repair, day7_bridge_repair},
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
                "5.2" => day5_2_print_queue(),
                "6" => day6_guard_gallivant(),
                "6.2" => day6_2_guard_gallivant(),
                "7" => day7_bridge_repair(),
                "7.2" => day7_2_bridge_repair(),
                _ => panic!("Not a valid day"),
            }
        }
    } else {
        println!("No extra parameters detected.");
    }
}
