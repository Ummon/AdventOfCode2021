use std::env;
use std::fs;
use std::time::Instant;

mod common;
mod day01;
mod day02;

fn day01() -> String {
    let report = common::read_list_of_numbers("data/day01.input", "\n");
    format!("part1: {}, part2: {}", day01::count_number_of_decreased_values(&report, 1), day01::count_number_of_decreased_values(&report, 3))
}

fn day02() -> String {
    let movements = day02::parse_movements(&fs::read_to_string("data/day02.input").unwrap());
    let final_position = day02::get_final_position(&movements);
    format!("part1: {}, part2: {}", final_position.horizontal * final_position.aim, final_position.horizontal * final_position.depth)
}

fn format_micros(t: u128) -> String {
    if t < 10_000 {
        format!("{} μs", t)
    } else if t < 10_000_000u128 {
        format!("{} ms", t / 1_000u128)
    } else {
        format!("{} s", t / 1_000_000u128)
    }
}

fn do_day(days: &[fn() -> String], day: usize) {
    let now = Instant::now();
    println!("Result of day {:02}: {} (time: {})", day, days[day - 1](), format_micros(now.elapsed().as_micros()));
}

fn main() {
    println!("https://adventofcode.com/2021");

    let days: Vec<fn() -> String> = vec!(
        day01,
        day02,
    );

    let args: Vec<String> = env::args().skip(1).collect();

    // No argument -> execute all day problems.
    if args.is_empty() {
        let now = Instant::now();
        for i in 1 ..= days.len() {
            do_day(&days, i)
        }
        println!("Time to execute all days: {}", format_micros(now.elapsed().as_micros()));
    } else {
        for arg in args {
            match arg.parse::<usize>() {
                Ok(day) if day >= 1 && day <= days.len() =>
                    do_day(&days, day),
                Ok(day) =>
                    println!("Unknown day: {}", day),
                Err(error) =>
                    println!("Unable to parse day number: \"{}\", error: {}", arg, error)
            }
        }
    }
}
