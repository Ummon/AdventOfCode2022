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

fn rock_collide(pos: (i64, i64), rock: &[(i64, i64)], pile: &HashSet<(i64, i64)>) -> bool {
    for (x, y) in rock {
        let (x, y) = (x + pos.0, y + pos.1);
        if x <= 0 || x >= 8 || y <= 0 || pile.contains(&(x, y)) {
            return true;
        }
    }
    false
}

#[derive(Debug)]
struct State {
    rock_type: usize,
    x_position: i64,
    highest_point: i64,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.rock_type == other.rock_type && self.x_position == other.x_position
    }
}

pub fn height(number_of_rocks: i64, movements: &[Movement]) -> i64 {
    let types_of_rock = [
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],         // '-'.
        vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)], // '+'.
        vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)], // '⅃'.
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],         // '|'.
        vec![(0, 0), (0, 1), (1, 0), (1, 1)],         // '□'.
    ];

    let l = movements.len();
    let mut move_states = Vec::<Option<State>>::new();
    let mut pile = HashSet::<(i64, i64)>::new();
    let mut current_movement = 0;
    let mut highest_point = 0;
    for i in 0..number_of_rocks {
        let rock_type = i as usize % types_of_rock.len();
        let rock = &types_of_rock[rock_type];
        let mut pos = (3, highest_point + 4);

        loop {
            let m = &movements[current_movement];
            current_movement = (current_movement + 1) % movements.len();

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

                highest_point = highest_point.max(pos.1 + h);

                move_states.push(Some(State {
                    rock_type,
                    x_position: pos.0,
                    highest_point,
                }));

                for k in 1..=move_states.len() / (2 * l) {
                    let sequence_length = k * l;
                    let sequence_start = move_states.len() - sequence_length;
                    if let Some(Some(previous_first_of_sequence)) =
                        move_states.get(sequence_start - 1)
                    {
                        if move_states[sequence_start..]
                            == move_states[sequence_start - sequence_length
                                ..move_states.len() - sequence_length]
                        {
                            let nb_stacked_rocks = move_states[sequence_start..]
                                .iter()
                                .fold(0, |sum, current| {
                                    sum + if current.is_some() { 1 } else { 0 }
                                });

                            let nb_stacked_rocks_remaining = number_of_rocks % nb_stacked_rocks;

                            return number_of_rocks / nb_stacked_rocks
                                * (highest_point - previous_first_of_sequence.highest_point)
                                + move_states
                                    .iter()
                                    .filter_map(|s| s.as_ref())
                                    .take(nb_stacked_rocks_remaining as usize)
                                    .last()
                                    .unwrap()
                                    .highest_point;
                        }
                    }
                }

                break;
            } else {
                pos = new_pos;
                move_states.push(None);
            }
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
    fn part2() {
        let movements = parse(JET_PATTERN);
        assert_eq!(height(1_000_000_000_000, &movements), 1_514_285_714_288);
    }
}
