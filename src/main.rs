#![allow(clippy::needless_return)]

use std::env;

use days::{
    day10_hoof_it::{day10_2_hoof_it, day10_hoof_it},
    day11_plutonian_pebbles::{day11_2_plutonian_pebbles, day11_plutonian_pebbles},
    day12_garden_groups::{day12_2_garden_groups, day12_garden_groups},
    day1_historian_hysteria::{day1_2_historian_hysteria, day1_historian_hysteria},
    day2_red_nosed_reports::{day2_2_red_nosed_reports, day2_red_nosed_reports},
    day3_mull_it_over::{day3_2_mull_it_over, day3_mull_it_over},
    day4_ceres_search::{day4_2_ceres_search, day4_ceres_search},
    day5_print_queue::{day5_2_print_queue, day5_print_queue},
    day6_guard_gallivant::{day6_2_guard_gallivant, day6_guard_gallivant},
    day7_bridge_repair::{day7_2_bridge_repair, day7_bridge_repair},
    day8_resonant_collinearity::{day8_2_resonant_collinearity, day8_resonant_collinearity},
    day9_disk_fragmenter::{day9_2_disk_fragmenter, day9_disk_fragmenter},
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
                "8" => day8_resonant_collinearity(),
                "8.2" => day8_2_resonant_collinearity(),
                "9" => day9_disk_fragmenter(),
                "9.2" => day9_2_disk_fragmenter(),
                "10" => day10_hoof_it(),
                "10.2" => day10_2_hoof_it(),
                "11" => day11_plutonian_pebbles(),
                "11.2" => day11_2_plutonian_pebbles(),
                "12" => day12_garden_groups(),
                "12.2" => day12_2_garden_groups(),
                _ => panic!("Not a valid day"),
            }
        }
    } else {
        println!("No extra parameters detected.");
    }
}
