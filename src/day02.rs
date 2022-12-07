use std::io::BufRead;

#[derive(PartialEq, Clone, Copy)]
pub enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    pub fn battle(&self, other: &Shape) -> i32 {
        (match (self, other) {
            (Shape::Rock, Shape::Scissors) |
            (Shape::Paper, Shape::Rock) |
            (Shape::Scissors, Shape::Paper) => 6,
            (a, b) if a == b => 3,
            _ => 0,
        }) +
        match self { Shape::Rock => 1, Shape::Paper => 2, Shape::Scissors => 3, }
    }

    pub fn parse(str: &str) -> Self {
        match str {
            "A" | "X" => Shape::Rock,
            "B" | "Y" => Shape::Paper,
            "C" | "Z" => Shape::Scissors,
            _ => panic!("Unknown letter: {}", str)
        }
    }
}

pub fn read_shapes<R>(reader: R) -> Vec<(Shape, Shape)>
where
    R: BufRead
{
    let mut shapes: Vec<(Shape, Shape)> = Vec::new();
    for l in reader.lines() {
        let s: Vec<Shape> = l.unwrap().trim().split(' ').map(Shape::parse).collect();
        shapes.push((s[0], s[1]));
    }
    shapes
}

pub fn read_shapes_2<R>(reader: R) -> Vec<(Shape, Shape)>
where
    R: BufRead
{
    let mut shapes: Vec<(Shape, Shape)> = Vec::new();
    for l in reader.lines() {
        let l = l.unwrap();
        let l: Vec<&str> = l.trim().split(' ').collect();
        let s1: Shape = Shape::parse(l[0]);
        let s2 = match l[1] {
            "X" => match s1 { Shape::Rock => Shape::Scissors, Shape::Paper => Shape::Rock, Shape::Scissors => Shape::Paper, }, // Need to lose.
            "Z" => match s1 { Shape::Rock => Shape::Paper, Shape::Paper => Shape::Scissors, Shape::Scissors => Shape::Rock, }, // Need to win.
            _ => s1, // Draw.
        };

        shapes.push((s1, s2));
    }
    shapes
}

pub fn get_score(shapes: &[(Shape, Shape)]) -> i32 {
    shapes.iter().fold(0, |sum, (s1, s2)| sum + s2.battle(s1))
}

#[cfg(test)]
mod tests {
    use super::*;

    static STRATEGY_GUIDE: &str =
        "A Y
         B X
         C Z";

    #[test]
    fn part1() {
        assert_eq!(get_score(&read_shapes(STRATEGY_GUIDE.as_bytes())), 15);
    }

    #[test]
    fn part2() {
        assert_eq!(get_score(&read_shapes_2(STRATEGY_GUIDE.as_bytes())), 12);
    }
}