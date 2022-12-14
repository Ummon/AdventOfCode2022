use itertools::Itertools;

// 1000x1000 matrix.
const N: usize = 1000;
const M: usize = 1000;

pub struct Rocks {
    state: Box<[bool; N * M]>,
}

impl Rocks {
    fn new() -> Self {
        Rocks { state: Box::new([false; N * M]) }
    }

    fn get(&self, i: usize, j: usize) -> bool {
        self.state[N * i + j]
    }

    fn set(&mut self, i: usize, j: usize) {
        self.state[N * i + j] = true;
    }
}

pub fn parse(input: &str) -> (Rocks, usize) {
    let mut max_i = 0;
    let mut rocks = Rocks::new();
    for l in input.lines().map(|l| l.replace(" ", "")) {
        for ((i1, j1), (i2, j2)) in l
            .split("->")
            .map(|p| {
                let ji: Vec<&str> = p.split(',').collect();
                (ji[1].parse::<usize>().unwrap(), ji[0].parse::<usize>().unwrap())
            })
            .tuple_windows()
        {
            for i in i1.min(i2)..=i1.max(i2) {
                for j in j1.min(j2)..=j1.max(j2) {
                    rocks.set(i, j);
                    max_i = i.max(max_i);
                }
            }
        }
    }
    (rocks, max_i + 2)
}

pub fn poor_sand(mut rocks: Rocks, floor: usize) -> (i32, i32) {
    let mut n = 0;
    let mut first_n_touching_floor = 0;

    fn is_obstruct(i: usize, j: usize, rocks: &Rocks, floor: usize) -> bool {
        i >= floor || rocks.get(i, j)
    }

    loop {
        let (mut grain_i, mut grain_j) = (0, 500);

        if rocks.get(grain_i, grain_j) {
            return (first_n_touching_floor, n);
        }

        loop {
            let grain_i_next = grain_i + 1;

            if first_n_touching_floor == 0 && grain_i_next >= floor {
                first_n_touching_floor = n;
            }

            if !is_obstruct(grain_i_next, grain_j, &rocks, floor) {
                (grain_i, grain_j) = (grain_i_next, grain_j);
            } else if !is_obstruct(grain_i_next, grain_j - 1, &rocks, floor) {
                (grain_i, grain_j) = (grain_i_next, grain_j - 1);
            } else if !is_obstruct(grain_i_next, grain_j + 1, &rocks, floor) {
                (grain_i, grain_j) = (grain_i_next, grain_j + 1);
            } else {
                rocks.set(grain_i, grain_j);
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
