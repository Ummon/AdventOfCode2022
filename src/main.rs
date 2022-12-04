use std::{env, fs, time::Instant, io::{BufReader, Seek, SeekFrom}};

mod common;
mod day01;
mod day02;
mod day03;
mod day04;

fn day01() -> String {
    let f = fs::File::open("data/day01.input").unwrap();
    let calories = day01::read_calories(BufReader::new(f));
    format!("part1: {}, part2: {}", day01::get_most_calories(&calories), day01::get_sum_most_three_calories(&calories))
}

fn day02() -> String {
    let mut f = fs::File::open("data/day02.input").unwrap();
    let shapes = day02::read_shapes(BufReader::new(f.try_clone().unwrap()));
    let _ = f.seek(SeekFrom::Start(0));
    let shapes_2 = day02::read_shapes_2(BufReader::new(f));
    format!("part1: {}, part2: {}", day02::get_score(&shapes), day02::get_score(&shapes_2))
}

fn day03() -> String {
    let rucksacks = day03::parse(&fs::read_to_string("data/day03.input").unwrap());
    format!("part1: {}, part2: {}", day03::priority_sum(&rucksacks), day03::badge_sum(&rucksacks))
}

fn day04() -> String {
    let pairs = day04::parse(&fs::read_to_string("data/day04.input").unwrap());
    format!("part1: {}, part2: {}", day04::number_fully_contain(&pairs), day04::number_overlaps(&pairs))
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
        day02,
        day03,
        day04,
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
