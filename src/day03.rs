use std::io::BufRead;

use itertools::Itertools;

pub fn parse(s: &str) -> Vec<Vec<u8>> {
    s.lines()
        .map(|l| {
            l.as_bytes().into_iter().map(letter_to_priority).collect_vec()
        }).collect()
}

fn letter_to_priority(v: &u8) -> u8 {
    if *v >= 97 { // >= 'a'
        v - 96
    } else { // < 'a'
        v - 38
    }
}

pub fn priority_sum(rucksacks: &[Vec<u8>]) -> i32 {
    let mut sum = 0;
    let mut item_count = [0u8; 52];
    for r in rucksacks {
        item_count.fill(0);

        for i in 0..r.len() / 2 {
            item_count[r[i] as usize - 1] += 1;
        }

        for i in r.len() / 2..r.len() {
            if item_count[r[i] as usize - 1] > 0 {
                sum += r[i] as i32;
                break;
            }
        }
    }
    sum
}

pub fn badge_sum(rucksacks: &[Vec<u8>]) -> i32 {
    let mut sum = 0;
    let mut item_set = [0u8; 52];
    for group in rucksacks.chunks(3).into_iter() {
        item_set.fill(0);

        for i in 0..2 {
            for b in group[i].iter() {
                item_set[*b as usize - 1] |= 1 << i;
            }
        }

        for b in group[2].iter() {
            if item_set[*b as usize - 1] == 3 {
                sum += *b as i32;
                break;
            }
        }
    }
    sum
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

        assert_eq!(priority_sum(&parse(rucksacks)), 157);
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

        assert_eq!(badge_sum(&parse(rucksacks)), 70);
    }
}