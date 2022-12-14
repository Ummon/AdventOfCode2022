use std::collections::HashSet;

use itertools::Itertools;

type Rocks = HashSet<(i32, i32)>;

pub fn parse(input: &str) -> (Rocks, i32) {
    let mut max_i = 0;
    let mut rocks = Rocks::new();
    for l in input.lines().map(|l| l.replace(" ", "")) {
        for ((i1, j1), (i2, j2)) in l
            .split("->")
            .map(|p| {
                let ji: Vec<&str> = p.split(',').collect();
                (ji[1].parse::<i32>().unwrap(), ji[0].parse::<i32>().unwrap())
            })
            .tuple_windows()
        {
            for i in i1.min(i2)..=i1.max(i2) {
                for j in j1.min(j2)..=j1.max(j2) {
                    rocks.insert((i, j));
                    max_i = i.max(max_i);
                }
            }
        }
    }
    (rocks, max_i + 2)
}

pub fn poor_sand(mut rocks: Rocks, floor: i32) -> (i32, i32) {
    let mut n = 0;
    let mut first_n_touching_floor = 0;

    fn is_obstruct(i: i32, j: i32, rocks: &Rocks, floor: i32) -> bool {
        i >= floor || rocks.contains(&(i, j))
    }

    loop {
        let mut grain = (0, 500);

        if rocks.contains(&grain) {
            return (first_n_touching_floor, n);
        }

        loop {
            if first_n_touching_floor == 0 && grain.0 + 1 >= floor {
                first_n_touching_floor = n;
            }

            if !is_obstruct(grain.0 + 1, grain.1, &rocks, floor) {
                grain = (grain.0 + 1, grain.1);
            } else if !is_obstruct(grain.0 + 1, grain.1 - 1, &rocks, floor) {
                grain = (grain.0 + 1, grain.1 - 1);
            } else if !is_obstruct(grain.0 + 1, grain.1 + 1, &rocks, floor) {
                grain = (grain.0 + 1, grain.1 + 1);
            } else {
                rocks.insert(grain);
                break;
            }
        }
        n = n + 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static ROCKS: &str = "498,4 -> 498,6 -> 496,6
        503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn part1() {
        let (rocks, floor) = parse(ROCKS);
        assert_eq!(poor_sand(rocks, floor).0, 24);
    }

    #[test]
    fn part2() {
        let (rocks, floor) = parse(ROCKS);
        assert_eq!(poor_sand(rocks, floor).1, 93);
    }
}
