use std::{env, time::Instant};

mod common;
mod days;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;

fn main() {
    println!("https://adventofcode.com/2022");

    let days: Vec<fn() -> String> = vec!(
        days::day01,
        days::day02,
        days::day03,
        days::day04,
        days::day05,
        days::day06,
        days::day07,
        days::day08,
        days::day09,
        days::day10,
        days::day11,
        days::day12,
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

fn do_day(days: &[fn() -> String], day: usize) {
    let now = Instant::now();
    println!("Result of day {:02}: {} (time: {})", day, days[day - 1](), format_micros(now.elapsed().as_micros()));
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
