use std::collections::HashMap;

use itertools::Itertools;
use regex::{self, Regex};

#[derive(Debug)]
pub struct Valve {
    neighbours: Vec<i32>,
    flow: i32,
}

pub fn parse(input: &str) -> (i32, Vec<Valve>) {
    let regex =
        Regex::new(r"Valve (\w{2}) has flow rate=(\d+); tunnels? leads? to valves? (.+)").unwrap();

    let mut valve_aa = 0;

    let mut names = HashMap::<String, i32>::new();
    let lines = input
        .lines()
        .enumerate()
        .map(|(i, l)| {
            let captures = regex.captures(l).unwrap();
            let i = i as i32;

            if &captures[1] == "AA" {
                valve_aa = i;
            }

            names.insert(captures[1].to_string(), i);

            (
                captures[2].parse::<i32>().unwrap(),
                captures[3]
                    .split(", ")
                    .map(str::to_string)
                    .collect::<Vec<String>>(),
            )
        })
        .collect_vec();

    let mut valves = Vec::new();

    for (flow, neigbours) in lines {
        valves.push(Valve {
            neighbours: neigbours.iter().map(|n| names[n]).collect(),
            flow,
        });
    }

    (valve_aa, valves)
}

pub fn most_pressure(start: i32, time: i32, valves: &[Valve]) -> i32 {
    let n = valves.len();
    let mut times_tables = vec![vec![0; n]; n];
    for i in 0..valves.len() {
        let mut visited = vec![false; n];
        let mut to_visit = Vec::new();
        let mut next_to_visit = vec![i as i32];
        let mut l = 0;
        while !next_to_visit.is_empty() {
            std::mem::swap(&mut to_visit, &mut next_to_visit);
            while let Some(n) = to_visit.pop() {
                if !visited[n as usize] {
                    visited[n as usize] = true;
                    times_tables[i as usize][n as usize] = l;
                    next_to_visit.extend_from_slice(&valves[n as usize].neighbours)
                }
            }
            l += 1;
        }
    }

    let non_broken_valves: Vec<i32> = valves
        .iter()
        .enumerate()
        .filter_map(|(i, v)| if v.flow > 0 { Some(i as i32) } else { None })
        .collect();

    fn best_score(
        time_left: i32,
        current_path: Vec<i32>,
        non_broken_valves: &[i32],
        times_tables: &[Vec<i32>],
        valves: &[Valve],
    ) -> i32 {
        non_broken_valves
            .iter()
            .map(|v| {
                if !current_path.contains(v) {
                    let time = times_tables[*current_path.last().unwrap() as usize][*v as usize];
                    // -1 is the time to open the valve.
                    let time_left_after_v = time_left - time - 1;
                    if time_left_after_v > 0 {
                        let mut path_cloned = current_path.clone();
                        path_cloned.push(*v);
                        time_left_after_v * valves[*v as usize].flow
                            + best_score(
                                time_left_after_v,
                                path_cloned,
                                non_broken_valves,
                                times_tables,
                                valves,
                            )
                    } else {
                        0
                    }
                } else {
                    0
                }
            })
            .max()
            .unwrap()
    }

    best_score(time, vec![start], &non_broken_valves, &times_tables, valves)
}

#[cfg(test)]
mod tests {
    use super::*;

    static VALVES: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
    Valve BB has flow rate=13; tunnels lead to valves CC, AA
    Valve CC has flow rate=2; tunnels lead to valves DD, BB
    Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
    Valve EE has flow rate=3; tunnels lead to valves FF, DD
    Valve FF has flow rate=0; tunnels lead to valves EE, GG
    Valve GG has flow rate=0; tunnels lead to valves FF, HH
    Valve HH has flow rate=22; tunnel leads to valve GG
    Valve II has flow rate=0; tunnels lead to valves AA, JJ
    Valve JJ has flow rate=21; tunnel leads to valve II";

    #[test]
    fn part1() {
        let (start, valves) = parse(VALVES);
        assert_eq!(most_pressure(start, 30, &valves), 1651);
    }

    #[test]
    fn part2() {}
}
