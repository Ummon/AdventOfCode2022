use std::collections::HashSet;

enum Direction {
    Left,
    Up,
    Right,
    Down,
}

pub struct Movement {
    direction: Direction,
    distance: i32,
}

pub fn parse(input: &str) -> Vec<Movement> {
    input.lines().map(|l| {
        let split: Vec<&str> = l.trim().split(' ').collect();
        Movement {
            direction: match split[0] {
                "L" => Direction::Left,
                "U" => Direction::Up,
                "R" => Direction::Right,
                "D" => Direction::Down,
                other => panic!("Uknown movement: {}", other),
            },
            distance: split[1].parse().expect("Can't parse distance"),
        }
    }).collect()
}

pub fn nb_positions_visited_by_tail<const N: usize>(movements: &[Movement]) -> usize {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert((0, 0));

    let mut rope = [(0, 0); N]; // First element is the tail, last element is the head.

    for m in movements {
        for _ in 0..m.distance {
            // 1) Move the head.
            let h = &mut rope[N-1];
            *h = match m.direction {
                Direction::Left => (h.0 - 1, h.1),
                Direction::Up => (h.0, h.1 - 1),
                Direction::Right => (h.0 + 1, h.1),
                Direction::Down => (h.0, h.1 + 1),
            };

            // 2) Move the rest of the rope.
            for i in (0..N-1).rev() {
                let target = rope[i+1];
                let mut node = rope[i];

                let (dx, dy): (i32, i32) = (node.0 - target.0, node.1 - target.1);
                let (dx_abs, dy_abs) = (dx.abs(), dy.abs());

                if dx_abs == 2 && dy_abs == 2 {
                    node = (target.0 + dx.signum(), target.1 + dy.signum());
                } else if dx_abs >= 2 {
                    node = (target.0 + dx.signum(), target.1);
                } else if dy_abs >= 2 {
                    node = (target.0, target.1 + dy.signum());
                }

                if i == 0 && node != rope[i] {
                    visited.insert(node);
                }

                rope[i] = node;
            }
        }
    };
    visited.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    static MOVEMENTS: &str =
        "R 4
         U 4
         L 3
         D 1
         R 4
         D 1
         L 5
         R 2";

    #[test]
    fn part1() {
        let movements = parse(MOVEMENTS);
        assert_eq!(nb_positions_visited_by_tail::<2>(&movements), 13);
    }

    #[test]
    fn part2() {
        let movements = parse(MOVEMENTS);
        assert_eq!(nb_positions_visited_by_tail::<10>(&movements), 1);

        let movements_2 = parse(
            "R 5
             U 8
             L 8
             D 3
             R 17
             D 10
             L 25
             U 20");
        assert_eq!(nb_positions_visited_by_tail::<10>(&movements_2), 36);
    }
}