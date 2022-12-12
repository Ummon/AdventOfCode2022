use std::{collections::VecDeque, str::Lines};

use itertools::Itertools;

#[derive(Debug, Clone)]
enum Operation {
    MulOld,
    Mul(u64),
    Add(u64),
}

impl Operation {
    fn parse(op: &[&str]) -> Self {
        match op[1] {
            "*" => {
                if op[2] == "old" {
                    Operation::MulOld
                } else {
                    Operation::Mul(op[2].parse::<u64>().unwrap())
                }
            }
            "+" => Operation::Add(op[2].parse::<u64>().unwrap()),
            unknown => panic!("Unknown operation: {}", unknown),
        }
    }

    fn apply(&self, v: u64) -> u64 {
        match self {
            Operation::MulOld => v * v,
            Operation::Mul(v2) => v * v2,
            Operation::Add(v2) => v + v2,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    divisible_test: u64,
    monkey_to_throw_if_true: usize,
    monkey_to_throw_if_false: usize,
}

static CHARS_SPLIT: &[char] = &[' ', ','];

impl Monkey {
    fn parse(lines: &mut Lines) -> Option<Self> {
        fn split_line(s: &str) -> impl Iterator<Item = &str> {
            s.trim().split(CHARS_SPLIT).filter(|s| !s.is_empty())
        }

        lines.next()?; // Skip first line (Monkey number).

        Some(Monkey {
            items: split_line(lines.next()?)
                .skip(2)
                .map(|v| v.parse::<u64>().unwrap())
                .collect(),
            operation: Operation::parse(&split_line(lines.next()?).skip(3).collect::<Vec<&str>>()),
            divisible_test: split_line(lines.next()?)
                .nth(3)
                .unwrap()
                .parse::<u64>()
                .unwrap(),
            monkey_to_throw_if_true: split_line(lines.next()?).nth(5).unwrap().parse().unwrap(),
            monkey_to_throw_if_false: split_line(lines.next()?).nth(5).unwrap().parse().unwrap(),
        })
    }
}

pub fn parse(input: &str) -> Vec<Monkey> {
    let mut monkeys = Vec::new();
    let mut lines = input.lines();
    while let Some(m) = Monkey::parse(&mut lines) {
        monkeys.push(m);
        lines.next(); // Empty line.
    }
    monkeys
}

pub fn run(monkeys: &mut [Monkey], nb_rounds: u64, worry_divided: u64) -> u64 {
    let mut inspected = vec![0u64; monkeys.len()];

    let base = monkeys
        .iter()
        .fold(1, |product, m| product * m.divisible_test);

    for _ in 0..nb_rounds {
        for i in 0..monkeys.len() {
            while let Some(item) = monkeys[i].items.pop_front() {
                inspected[i] += 1;
                let new_worry = (monkeys[i].operation.apply(item) / worry_divided) % base;
                if new_worry % monkeys[i].divisible_test == 0 {
                    monkeys[monkeys[i].monkey_to_throw_if_true]
                        .items
                        .push_back(new_worry);
                } else {
                    monkeys[monkeys[i].monkey_to_throw_if_false]
                        .items
                        .push_back(new_worry);
                }
            }
        }
    }

    inspected.iter().sorted().rev().take(2).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    static MONKEYS: &str = "Monkey 0:
         Starting items: 79, 98
         Operation: new = old * 19
         Test: divisible by 23
           If true: throw to monkey 2
           If false: throw to monkey 3

         Monkey 1:
           Starting items: 54, 65, 75, 74
           Operation: new = old + 6
           Test: divisible by 19
             If true: throw to monkey 2
             If false: throw to monkey 0

         Monkey 2:
           Starting items: 79, 60, 97
           Operation: new = old * old
           Test: divisible by 13
             If true: throw to monkey 1
             If false: throw to monkey 3

         Monkey 3:
           Starting items: 74
           Operation: new = old + 3
           Test: divisible by 17
             If true: throw to monkey 0
             If false: throw to monkey 1";

    #[test]
    fn part1() {
        let mut monkeys = parse(MONKEYS);
        assert_eq!(run(&mut monkeys, 20, 3), 10605);
    }

    #[test]
    fn part2() {
        let mut monkeys = parse(MONKEYS);
        assert_eq!(run(&mut monkeys, 10000, 1), 2713310158);
    }
}
