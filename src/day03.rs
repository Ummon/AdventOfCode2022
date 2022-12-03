use std::io::{BufRead, Lines};

use itertools::Itertools;

pub fn priority_sum<R>(reader: R) -> i32
where
    R: BufRead
{
    let mut sum = 0;
    let mut item_count = [0i32; 52];
    for l in reader.lines() {
        item_count.fill(0);
        let l = l.unwrap();
        let items = l.as_bytes();
        for i in 0..items.len() / 2 {
            let v = letter_to_priority(items[i]);
            item_count[v as usize - 1] += 1;
        }
        for i in items.len() / 2..items.len() {
            let v = letter_to_priority(items[i]);
            if item_count[v as usize - 1] > 0 {
                sum += v as i32;
                break;
            }
        }
    }
    sum
}

pub fn badge_sum<R>(reader: R) -> i32
where
    R: BufRead
{
    let mut sum = 0;
    let mut item_count = [0i32; 52];
    for group in reader.lines().chunks(3).into_iter() {
        item_count.fill(0);
        for (i, rucksack) in group.enumerate() {
            let items = rucksack.unwrap();
            for b in items.as_bytes().into_iter() {
                let v = letter_to_priority(*b);
                if i == 2 && item_count[v as usize - 1] == 3 {
                    sum += v as i32;
                    break;
                }
                item_count[v as usize - 1] |= 1 << i;
            }
        }
    }
    sum
}

fn letter_to_priority(v: u8) -> u8 {
    if v >= 97 { // >= 'a'
        v - 96
    } else { // < 'a'
        v - 38
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let rucksacks =
        "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        assert_eq!(priority_sum(rucksacks.as_bytes()), 157);
    }

    #[test]
    fn part2() {
        let rucksacks =
        "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        assert_eq!(badge_sum(rucksacks.as_bytes()), 70);
    }
}