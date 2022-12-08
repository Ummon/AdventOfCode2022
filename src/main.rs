use std::{env, fs, time::Instant, io::{BufReader, Seek, SeekFrom}};

mod common;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;

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

fn day05() -> String {
    let (mut stacks, moves) = day05::parse(&fs::read_to_string("data/day05.input").unwrap());
    let mut stacks2 = stacks.clone();
    day05::apply_moves_by_crate_mover_9000(&mut stacks, &moves);
    day05::apply_moves_by_crate_mover_9001(&mut stacks2, &moves);
    format!("part1: {}, part2: {}", day05::get_top_as_string(&stacks), day05::get_top_as_string(&stacks2))
}

fn day06() -> String {
    let signals = fs::read_to_string("data/day06.input").unwrap();
    format!("part1: {}, part2: {}", day06::first_marker_pos(&signals, 4), day06::first_marker_pos(&signals, 14))
}

fn day07() -> String {
    let root = day07::parse(&fs::read_to_string("data/day07.input").unwrap());

    let (root_size, sum_part1, ) = {
        let mut sizes: Vec<i64> = Vec::new();
        (root.dir_sizes(|size| size <= 100_000, &mut sizes), sizes.iter().sum::<i64>())
    };

    let min_part2 = {
        let to_free = root_size - (70_000_000 - 30_000_000);
        let mut sizes: Vec<i64> = Vec::new();
        root.dir_sizes(|size| size >= to_free, &mut sizes);
        *sizes.iter().min().unwrap()
    };

    format!("part1: {}, part2: {}", sum_part1, min_part2)
}

fn day08() -> String {
    let forest = day08::parse(&fs::read_to_string("data/day08.input").unwrap());
    format!("part1: {}, part2: {}", day08::number_of_visible_trees(&forest), day08::best_scenic_score(&forest))
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
        day05,
        day06,
        day07,
        day08,
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
