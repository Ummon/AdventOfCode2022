use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
pub struct Sensor {
    x: i64,
    y: i64,
    radius: i64,
}

#[derive(PartialEq)]
pub struct Beacon {
    x: i64,
    y: i64,
}

pub fn parse(input: &str) -> (Vec<Sensor>, Vec<Beacon>) {
    let regex =
        Regex::new(r"Sensor at x=(-?{1}\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
            .unwrap();
    let mut sensors = Vec::new();
    let mut beacons = Vec::new();
    for l in input.lines() {
        let captures = regex.captures(l).unwrap();

        let (s_x, s_y) = (
            captures[1].parse::<i64>().unwrap(),
            captures[2].parse::<i64>().unwrap(),
        );
        let (b_x, b_y) = (
            captures[3].parse::<i64>().unwrap(),
            captures[4].parse::<i64>().unwrap(),
        );

        sensors.push(Sensor {
            x: s_x,
            y: s_y,
            radius: (s_x - b_x).abs() + (s_y - b_y).abs(),
        });

        let beacon = Beacon { x: b_x, y: b_y };
        if !beacons.contains(&beacon) {
            beacons.push(Beacon { x: b_x, y: b_y });
        }
    }

    (sensors, beacons)
}

pub fn number_of_position_without_beacon(sensors: &[Sensor], beacons: &[Beacon], row: i64) -> i64 {
    let nb_beacons_on_row = beacons
        .into_iter()
        .filter_map(|b| if b.y == row { Some(b.x) } else { None })
        .count() as i64;

    let segments = sensors
        .into_iter()
        .filter_map(|s| {
            let dx = s.radius - (s.y - row).abs();
            if dx >= 0 {
                Some((s.x - dx, s.x + dx))
            } else {
                None
            }
        })
        .sorted();

    [(i64::MIN, i64::MIN)]
        .into_iter()
        .chain(segments)
        .tuple_windows()
        .fold(-nb_beacons_on_row + 1, |sum, (seg1, seg2)| {
            sum + if seg2.0 > seg1.1 {
                seg2.1 - seg2.0
            } else if seg2.1 > seg1.1 {
                seg2.1 - seg1.1
            } else {
                0
            }
        })
}

pub fn tuning_frequency(sensors: &[Sensor], limit: i64) -> i64 {
    for s in sensors.iter() {
        for x in s.x - s.radius - 1..=s.x + s.radius + 1 {
            if x > limit {
                break;
            } else if x < 0 {
                continue;
            }

            let dy = s.radius - (x - s.x).abs() + 1;
            'a: for y in [s.y + dy, s.y - dy] {
                if y <= limit && y >= 0 {
                    for s2 in sensors.iter() {
                        if (s2.x - x).abs() + (s2.y - y).abs() <= s2.radius {
                            break 'a;
                        }
                    }
                    return x * 4_000_000 + y;
                }
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    static SCAN: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
    Sensor at x=9, y=16: closest beacon is at x=10, y=16
    Sensor at x=13, y=2: closest beacon is at x=15, y=3
    Sensor at x=12, y=14: closest beacon is at x=10, y=16
    Sensor at x=10, y=20: closest beacon is at x=10, y=16
    Sensor at x=14, y=17: closest beacon is at x=10, y=16
    Sensor at x=8, y=7: closest beacon is at x=2, y=10
    Sensor at x=2, y=0: closest beacon is at x=2, y=10
    Sensor at x=0, y=11: closest beacon is at x=2, y=10
    Sensor at x=20, y=14: closest beacon is at x=25, y=17
    Sensor at x=17, y=20: closest beacon is at x=21, y=22
    Sensor at x=16, y=7: closest beacon is at x=15, y=3
    Sensor at x=14, y=3: closest beacon is at x=15, y=3
    Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn part1() {
        let (sensors, beacons) = parse(SCAN);
        assert_eq!(
            number_of_position_without_beacon(&sensors, &beacons, 10),
            26
        );
    }

    #[test]
    fn part2() {
        let (sensors, _) = parse(SCAN);
        assert_eq!(tuning_frequency(&sensors, 20), 56_000_011);
    }
}
