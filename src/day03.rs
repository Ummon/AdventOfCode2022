use std::io::BufRead;

pub fn read_<R>(reader: R) -> Vec<i32>
where
    R: BufRead
{
    /*
    let mut shapes: Vec<(Shape, Shape)> = Vec::new();
    for l in reader.lines() {
        let s: Vec<Shape> = l.unwrap().trim().split(' ').map(Shape::parse).collect();
        shapes.push((s[0], s[1]));
    }
    shapes
     */
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(1, 1);
    }

    #[test]
    fn part2() {
        assert_eq!(2, 2);
    }
}