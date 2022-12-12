use std::{cmp::Reverse, io::BufRead, iter::Iterator, ops::AddAssign};

use itertools::*;

pub fn read_calories<R>(reader: R) -> Vec<i64>
where
    R: BufRead,
{
    let mut calories = vec![0i64];
    for l in reader.lines() {
        let l = l.unwrap();
        let trimmed = l.trim();
        if trimmed == "" {
            calories.push(0);
        } else {
            calories
                .last_mut()
                .unwrap()
                .add_assign(trimmed.parse::<i64>().unwrap());
        }
    }
    calories
}

pub fn get_most_calories(calories: &[i64]) -> i64 {
    *(calories.iter().max().unwrap())
}

pub fn get_sum_most_three_calories(calories: &[i64]) -> i64 {
    calories
        .iter()
        .map(Reverse)
        .k_smallest(3)
        .fold(0, |s, v| s + v.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    static CALORIES: &str = "1000
        2000
        3000

        4000

        5000
        6000

        7000
        8000
        9000

        10000";

    #[test]
    fn part1() {
        assert_eq!(
            get_most_calories(&read_calories(CALORIES.as_bytes())),
            24000
        );
    }

    #[test]
    fn part2() {
        assert_eq!(
            get_sum_most_three_calories(&read_calories(CALORIES.as_bytes())),
            45000
        );
    }
}
