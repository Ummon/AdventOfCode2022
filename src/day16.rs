use std::collections::HashMap;

use itertools::Itertools;
use regex::{self, Regex};

const MAX_TRAVEL_TIME_FROM_ONE_VALVE_TO_ANOTHER: i32 = 9; // [min].

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

pub fn most_pressure(start: i32, time: i32, nb_people: i32, valves: &[Valve]) -> i32 {
    // TODO: use Floyd-Warshall algorithm.
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

    #[derive(Debug, Clone, Copy)]
    struct Node {
        valve: i32,
        time_opened: i32,
    }

    fn best_score(
        total_time: i32,
        current_nodes: &[Node],
        unvisited_non_broken_valves: &[i32],
        times_tables: &[Vec<i32>],
        valves: &[Valve],
    ) -> i32 {
        let pressure_released: i32 = current_nodes
            .iter()
            .map(|node| (total_time - node.time_opened) * valves[node.valve as usize].flow)
            .sum();

        let next_valves = unvisited_non_broken_valves
            .iter()
            .permutations(current_nodes.len());

        pressure_released
            + next_valves
                .map(|vs| {
                    let mut next_nodes: Vec<Node> = Vec::new();
                    for i in 0..vs.len() {
                        let current_node = current_nodes[i];
                        let next_valve = *vs[i];

                        let added_time =
                            times_tables[current_node.valve as usize][next_valve as usize] + 1;

                        if added_time >= MAX_TRAVEL_TIME_FROM_ONE_VALVE_TO_ANOTHER {
                            return 0;
                        }

                        let time_opened = current_node.time_opened + added_time;

                        if time_opened < total_time {
                            next_nodes.push(Node {
                                valve: next_valve,
                                time_opened,
                            });
                        }
                    }
                    if !next_nodes.is_empty() {
                        best_score(
                            total_time,
                            &next_nodes,
                            &unvisited_non_broken_valves
                                .into_iter()
                                .map(|e| *e)
                                .filter(|v| !next_nodes.iter().any(|v2| *v == v2.valve))
                                .collect_vec(),
                            times_tables,
                            valves,
                        )
                    } else {
                        0
                    }
                })
                .max()
                .unwrap_or(0)
    }

    best_score(
        time,
        &(0..nb_people)
            .map(|_| Node {
                valve: start,
                time_opened: 0,
            })
            .collect_vec(),
        &non_broken_valves,
        &times_tables,
        &valves,
    )
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
        assert_eq!(most_pressure(start, 30, 1, &valves), 1651);
    }

    #[test]
    fn part2() {
        let (start, valves) = parse(VALVES);
        assert_eq!(most_pressure(start, 26, 2, &valves), 1707);
    }
}
