use std::collections::VecDeque;

use regex::Regex;

type Stacks = Vec<VecDeque<char>>;

pub struct Move {
    n: usize,
    from: usize,
    to: usize,
}

pub fn parse(s: &str) -> (Stacks, Vec<Move>) {
    let mut stacks = Vec::new();
    let mut lines = s.lines();
    while let Some(line) = lines.next() {
        let chars: Vec<char> = line.trim_end().chars().collect();
        if chars[1] == '1' {
            break;
        }
        let n = (chars.len() + 1) / 4;
        for _ in stacks.len()..n {
            stacks.push(VecDeque::new());
        }

        for i in 0..n {
            let pos_char = i * 4 + 1;
            if chars[pos_char] != ' ' {
                stacks[i].push_front(chars[pos_char]);
            }
        }
    }

    lines.next(); // Drop the empty line.

    let r = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let mut moves = Vec::new();
    while let Some(line) = lines.next() {
        let cap = r.captures(line).unwrap();
        moves.push(Move { n: cap[1].parse().unwrap(), from: cap[2].parse::<usize>().unwrap() - 1, to: cap[3].parse::<usize>().unwrap() - 1 });
    }

    (stacks, moves)
}

pub fn apply_moves_by_crate_mover_9000(stacks: &mut Stacks, moves: &[Move]) {
    for m in moves {
        for _ in 0..m.n {
            if let Some(c) = stacks[m.from].pop_back() {
                stacks[m.to].push_back(c);
            } else {
                break;
            }
        }
    }
}

pub fn apply_moves_by_crate_mover_9001(stacks: &mut Stacks, moves: &[Move]) {
    for m in moves {
        let from = stacks.get_mut(m.from).unwrap();
        let mut to_move = from.split_off(from.len() - m.n);
        stacks[m.to].append(&mut to_move);
    }
}

pub fn get_top_as_string(stacks: &Stacks) -> String {
    let mut result = String::new();
    for stack in stacks {
        if let Some(c) = stack.back() {
            result.push(*c);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let stacks_and_moves ="    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        let (mut stacks, moves) = parse(stacks_and_moves);
        apply_moves_by_crate_mover_9000(&mut stacks, &moves);
        assert_eq!(get_top_as_string(&stacks), "CMZ");
    }

    #[test]
    fn part2() {
        let stacks_and_moves ="    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        let (mut stacks, moves) = parse(stacks_and_moves);
        apply_moves_by_crate_mover_9001(&mut stacks, &moves);
        assert_eq!(get_top_as_string(&stacks), "MCD");
    }
}