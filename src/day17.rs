use std::collections::HashSet;

#[derive(Debug)]
pub enum Movement {
    Left,
    Right,
}

pub fn parse(input: &str) -> Vec<Movement> {
    input
        .chars()
        .map(|c| match c {
            '>' => Movement::Right,
            '<' => Movement::Left,
            other => panic!("Uknown movement: {}", other),
        })
        .collect()
}

fn rock_collide(pos: (i32, i32), rock: &[(i32, i32)], pile: &HashSet<(i32, i32)>) -> bool {
    for (x, y) in rock {
        let (x, y) = (x + pos.0, y + pos.1);
        if x <= 0 || x >= 8 || y <= 0 || pile.contains(&(x, y)) {
            return true;
        }
    }
    false
}

pub fn height(number_of_rocks: i32, movements: &[Movement]) -> i32 {
    let types_of_rock = [
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],         // '-'.
        vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)], // '+'.
        vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)], // '⅃'.
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],         // '|'.
        vec![(0, 0), (0, 1), (1, 0), (1, 1)],         // '□'.
    ];

    let mut pile = HashSet::<(i32, i32)>::new();
    let mut current_movement = 0;
    let mut highest_point = 0;
    for i in 0..number_of_rocks {
        let rock = &types_of_rock[i as usize % types_of_rock.len()];

        let mut pos = (3, highest_point + 4);
        loop {
            let m = &movements[current_movement];
            current_movement = (current_movement + 1) % movements.len();

            //println!("Rock: {:?}, pos: {:?}, m: {:?}", rock, pos, m);

            let new_pos = match m {
                Movement::Left => (pos.0 - 1, pos.1),
                Movement::Right => (pos.0 + 1, pos.1),
            };

            if !rock_collide(new_pos, rock, &pile) {
                pos = new_pos;
            }

            let new_pos = (pos.0, pos.1 - 1);
            if rock_collide(new_pos, rock, &pile) {
                let mut h = 0;
                for p in rock {
                    h = h.max(p.1);
                    pile.insert((p.0 + pos.0, p.1 + pos.1));
                }

                // println!("Rock piled up: {:?}, pos: {:?}", rock, pos);
                highest_point = highest_point.max(pos.1 + h);
                break;
            }

            pos = new_pos
        }
    }

    highest_point
}

#[cfg(test)]
mod tests {
    use super::*;

    static JET_PATTERN: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn part1() {
        let movements = parse(JET_PATTERN);
        assert_eq!(height(2022, &movements), 3068);
    }

    #[test]
    fn part2() {}
}
