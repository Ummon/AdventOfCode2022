use itertools::{FoldWhile::{Continue, Done}, Itertools};

#[derive(Clone, Copy)]
enum Orientation {
    West,
    North,
    Est,
    South,
}

#[derive(Debug)]
pub struct Matrix<T>(Vec<Vec<T>>);

impl<T> Matrix<T>
where
    T: Default + Clone
{
    fn new(h: usize, w: usize) -> Self {
        let mut m: Vec<Vec<T>> = Vec::new();
        for _ in 0..h {
            m.push(vec![T::default(); w]);
        }
        Matrix(m)
    }

    fn rotate(&self, i: usize, j: usize, o: Orientation) -> (usize, usize) {
        match o {
            Orientation::West => (i, j),
            Orientation::North => (j, self.height() - i - 1),
            Orientation::Est => (self.height() - i - 1, self.width() - j - 1),
            Orientation::South => (self.width() - j - 1, i),
        }
    }

    fn get(&self, i: usize, j: usize) -> &T {
        &self.0[i][j]
    }

    fn get_orientation(&self, i: usize, j: usize, o: Orientation) -> &T {
        let (i, j) = self.rotate(i, j, o);
        &self.0[i][j]
    }

    fn set_orientation(&mut self, i: usize, j: usize, o: Orientation, value: T) {
        let (i, j) = self.rotate(i, j, o);
        self.0[i][j] = value;
    }

    fn height(&self) -> usize {
        self.0.len()
    }

    fn width(&self) -> usize {
        self.0[0].len()
    }
}


pub fn parse(input: &str) -> Matrix<i32> {
    let mut m: Vec<Vec<i32>> = Vec::new();
    for l in input.lines() {
        let row: Vec<i32> = l.trim().chars().map(|c| c.to_digit(10).unwrap() as i32).collect();
        m.push(row);
    }
    Matrix(m)
}

// O(n).
pub fn number_of_visible_trees(forest: &Matrix<i32>) -> i32 {
    let h = forest.height();
    let w = forest.width();

    let mut visibility = Matrix::<bool>::new(h, w);
    let mut nb_visible_tree = 0;

    for o in [Orientation::West, Orientation::North, Orientation::Est, Orientation::South] {
        for i in 0..h {
            let mut max = -1;
            for j in 0..w {
                let tree_height = forest.get_orientation(i, j, o);
                if *tree_height > max {
                    if !visibility.get_orientation(i, j, o) {
                        visibility.set_orientation(i, j, o, true);
                        nb_visible_tree += 1
                    }
                    max = *tree_height;
                }
            }
        }
    }
    nb_visible_tree
}

pub fn best_scenic_score(forest: &Matrix<i32>) -> i32 {
    let h = forest.height();
    let w = forest.width();

    let mut current_best_score = -1;

    for i in 1..h-1 {
        for j in 1..w-1 {
            let current = forest.get(i, j);
            let dist_w = (1..j).rev().fold_while(1, |dist, j2| if forest.get(i, j2) >= current { Done(dist) } else { Continue(dist + 1) }).into_inner();
            let dist_n = (1..i).rev().fold_while(1, |dist, i2| if forest.get(i2, j) >= current { Done(dist) } else { Continue(dist + 1) }).into_inner();
            let dist_e = (j+1..w-1).fold_while(1, |dist, j2| if forest.get(i, j2) >= current { Done(dist) } else { Continue(dist + 1) }).into_inner();
            let dist_s = (i+1..h-1).fold_while(1, |dist, i2| if forest.get(i2, j) >= current { Done(dist) } else { Continue(dist + 1) }).into_inner();

            let score = dist_w * dist_n * dist_e * dist_s;
            if score > current_best_score {
                current_best_score = score;
            }
        }
    }

    current_best_score
}

#[cfg(test)]
mod tests {
    use super::*;

    static FOREST: &str =
        "30373
         25512
         65332
         33549
         35390";

    #[test]
    fn part1() {
        let forest = parse(FOREST);
        assert_eq!(number_of_visible_trees(&forest), 21)
    }

    #[test]
    fn part2() {
        let forest = parse(FOREST);
        assert_eq!(best_scenic_score(&forest), 8)
    }
}