use std::time::Instant;

use clap::Parser;
use rayon::prelude::*;

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
mod day13;
mod day14;
mod days;

#[derive(Parser, Debug)]
#[command(author = "Greg Burri", version = "1.0", about = "Advent of Code 2022")]
struct Args {
    #[arg(index(1), exclusive(true))]
    day: Option<usize>,

    #[arg(short, long)]
    parallel: bool,
}

fn main() {
    println!("https://adventofcode.com/2022");

    let days: Vec<fn() -> String> = vec![
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
        days::day13,
        days::day14,
    ];

    let args = Args::parse();

    match args.day {
        Some(day) => {
            if day >= 1 && day <= days.len() {
                do_day(&days, day)
            } else {
                println!("Unknown day: {}", day)
            }
        }
        // No argument -> execute all day problems.
        None => {
            let now = Instant::now();

            if args.parallel {
                (1..=days.len())
                    .into_par_iter()
                    .for_each(|d| do_day(&days, d));
            } else {
                (1..=days.len()).for_each(|d| do_day(&days, d));
            }

            println!(
                "Time to execute all days: {}",
                format_micros(now.elapsed().as_micros())
            );
        }
    }
}

fn do_day(days: &[fn() -> String], day: usize) {
    let now = Instant::now();
    println!(
        "Result of day {:02}: {} (time: {})",
        day,
        days[day - 1](),
        format_micros(now.elapsed().as_micros())
    );
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
