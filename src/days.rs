use std::{
    fs,
    io::{BufReader, Seek, SeekFrom},
};

use crate::*;

pub fn day01() -> String {
    let f = fs::File::open("data/day01.input").unwrap();
    let calories = day01::read_calories(BufReader::new(f));
    format!(
        "part1: {}, part2: {}",
        day01::get_most_calories(&calories),
        day01::get_sum_most_three_calories(&calories)
    )
}

pub fn day02() -> String {
    let mut f = fs::File::open("data/day02.input").unwrap();
    let shapes = day02::read_shapes(BufReader::new(f.try_clone().unwrap()));
    let _ = f.seek(SeekFrom::Start(0));
    let shapes_2 = day02::read_shapes_2(BufReader::new(f));
    format!(
        "part1: {}, part2: {}",
        day02::get_score(&shapes),
        day02::get_score(&shapes_2)
    )
}

pub fn day03() -> String {
    let rucksacks = day03::parse(&fs::read_to_string("data/day03.input").unwrap());
    format!(
        "part1: {}, part2: {}",
        day03::priority_sum(&rucksacks),
        day03::badge_sum(&rucksacks)
    )
}

pub fn day04() -> String {
    let pairs = day04::parse(&fs::read_to_string("data/day04.input").unwrap());
    format!(
        "part1: {}, part2: {}",
        day04::number_fully_contain(&pairs),
        day04::number_overlaps(&pairs)
    )
}

pub fn day05() -> String {
    let (mut stacks, moves) = day05::parse(&fs::read_to_string("data/day05.input").unwrap());
    let mut stacks2 = stacks.clone();
    day05::apply_moves_by_crate_mover_9000(&mut stacks, &moves);
    day05::apply_moves_by_crate_mover_9001(&mut stacks2, &moves);
    format!(
        "part1: {}, part2: {}",
        day05::get_top_as_string(&stacks),
        day05::get_top_as_string(&stacks2)
    )
}

pub fn day06() -> String {
    let signals = fs::read_to_string("data/day06.input").unwrap();
    format!(
        "part1: {}, part2: {}",
        day06::first_marker_pos(&signals, 4),
        day06::first_marker_pos(&signals, 14)
    )
}

pub fn day07() -> String {
    let root = day07::parse(&fs::read_to_string("data/day07.input").unwrap());

    let (root_size, sum_part1) = {
        let mut sizes: Vec<i64> = Vec::new();
        (
            root.dir_sizes(|size| size <= 100_000, &mut sizes),
            sizes.iter().sum::<i64>(),
        )
    };

    let min_part2 = {
        let to_free = root_size - (70_000_000 - 30_000_000);
        let mut sizes: Vec<i64> = Vec::new();
        root.dir_sizes(|size| size >= to_free, &mut sizes);
        *sizes.iter().min().unwrap()
    };

    format!("part1: {}, part2: {}", sum_part1, min_part2)
}

pub fn day08() -> String {
    let forest = day08::parse(&fs::read_to_string("data/day08.input").unwrap());
    format!(
        "part1: {}, part2: {}",
        day08::number_of_visible_trees(&forest),
        day08::best_scenic_score(&forest)
    )
}

pub fn day09() -> String {
    let movements = day09::parse(&fs::read_to_string("data/day09.input").unwrap());
    format!(
        "part1: {}, part2: {}",
        day09::nb_positions_visited_by_tail::<2>(&movements),
        day09::nb_positions_visited_by_tail::<10>(&movements)
    )
}

pub fn day10() -> String {
    let instructions = day10::parse(&fs::read_to_string("data/day10.input").unwrap());
    let mut screen = day10::Screen::new();
    let sum_signal_strength = screen.draw_screen(&instructions);
    format!(
        "part1: {}, part2: \n{}",
        sum_signal_strength,
        screen.to_ascii()
    )
}

pub fn day11() -> String {
    let monkeys = day11::parse(&fs::read_to_string("data/day11.input").unwrap());
    format!(
        "part1: {}, part2: {}",
        day11::run::<3>(&mut monkeys.clone(), 20),
        day11::run::<1>(&mut monkeys.clone(), 10000)
    )
}

pub fn day12() -> String {
    let heightmap = day12::parse(&fs::read_to_string("data/day12.input").unwrap());
    format!(
        "part1: {}, part2: {}",
        day12::nb_steps(&heightmap, day12::Path::StartToEnd),
        day12::nb_steps(&heightmap, day12::Path::EndTo0Elevation)
    )
}

pub fn day13() -> String {
    let signals = day13::parse(&fs::read_to_string("data/day13.input").unwrap());
    format!(
        "part1: {}, part2: {}",
        day13::sum_indices_signals_in_the_right_order(&signals),
        day13::product_indices_special_signals(&signals)
    )
}

pub fn day14() -> String {
    let (rocks, floor) = day14::parse(&fs::read_to_string("data/day14.input").unwrap());
    let (first_grain_touching_floor, last_grain) = day14::pour_sand(rocks, floor);
    format!(
        "part1: {}, part2: {}",
        first_grain_touching_floor, last_grain
    )
}

pub fn day15() -> String {
    let (sensors, beacons) = day15::parse(&fs::read_to_string("data/day15.input").unwrap());
    format!(
        "part1: {}, part2: {}",
        day15::number_of_position_without_beacon(&sensors, &beacons, 2_000_000),
        day15::tuning_frequency(&sensors, 4_000_000)
    )
}

pub fn day16() -> String {
    let (start, valves) = day16::parse(&fs::read_to_string("data/day16.input").unwrap());
    format!(
        "part1: {}, part2: {}",
        day16::most_pressure(start, 30, 1, &valves),
        day16::most_pressure(start, 26, 2, &valves)
    )
}

pub fn day17() -> String {
    let movements = day17::parse(&fs::read_to_string("data/day17.input").unwrap());
    format!(
        "part1: {}, part2: {}",
        day17::height(2022, &movements),
        day17::height(1_000_000_000_000, &movements)
    )
}

pub fn day18() -> String {
    let cubes = day18::parse(&fs::read_to_string("data/day18.input").unwrap());
    format!("part1: {}, part2: {}", day18::surface(&cubes), 0)
}
