use std::{env, fs, io, time::Instant};

mod common;
mod day01;

fn day01() -> String {
    //let report = common::read_list_of_numbers("data/day01.input", "\n");
    let f = fs::File::open("data/day01.input").unwrap();
    let calories = day01::read_calories(std::io::BufReader::new(f));
    format!("part1: {}, part2: {}", day01::get_most_calories(&calories), day01::get_sum_most_three_calories(&calories))
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
    println!("https://adventofcode.com/2022");

    let days: Vec<fn() -> String> = vec!(
        day01,
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
