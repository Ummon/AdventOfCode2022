#[derive(Debug)]
pub struct Heightmap {
    elevations: Vec<Vec<i32>>,
    start: (usize, usize),
    end: (usize, usize),
}

fn char_to_num(c: char) -> i32 {
    c as i32 - 'a' as i32
}

pub fn parse(input: &str) -> Heightmap {
    let mut hm = Heightmap {
        elevations: Vec::new(),
        start: (0, 0),
        end: (0, 0),
    };
    for (i, l) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (j, c) in l.trim().chars().enumerate() {
            if c == 'S' {
                hm.start = (i, j);
                row.push(char_to_num('a'));
            } else if c == 'E' {
                hm.end = (i, j);
                row.push(char_to_num('z'));
            } else {
                row.push(char_to_num(c));
            }
        }
        hm.elevations.push(row);
    }
    hm
}

#[derive(PartialEq)]
pub enum Path {
    StartToEnd,
    EndTo0Elevation,
}

pub fn nb_steps(hm: &Heightmap, path: Path) -> i32 {
    let (n, m) = (hm.elevations.len(), hm.elevations[0].len());
    let mut visited: Vec<Vec<i32>> = vec![vec![-1; m]; n];
    let mut positions = vec![hm.end];
    let mut next_positions = Vec::new();
    let mut step = -1;

    let neighbors = |i, j| {
        let mut positions = Vec::new();
        if i > 0 {
            positions.push((i - 1, j));
        }
        if i < n - 1 {
            positions.push((i + 1, j));
        }
        if j > 0 {
            positions.push((i, j - 1));
        }
        if j < m - 1 {
            positions.push((i, j + 1));
        }
        positions
    };

    visited[hm.end.0][hm.end.1] = 0;

    loop {
        step = step + 1;
        for (i, j) in positions.drain(..) {
            if path == Path::StartToEnd && (i, j) == hm.start
                || path == Path::EndTo0Elevation && hm.elevations[i][j] == 0
            {
                return step;
            }

            for (ni, nj) in neighbors(i, j) {
                if visited[ni][nj] == -1 && hm.elevations[i][j] - hm.elevations[ni][nj] <= 1 {
                    visited[ni][nj] = step;
                    next_positions.push((ni, nj));
                }
            }
        }
        std::mem::swap(&mut next_positions, &mut positions);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static HEIGHTMAP: &str = "Sabqponm
        abcryxxl
        accszExk
        acctuvwj
        abdefghi";

    #[test]
    fn part1() {
        let heightmap = parse(HEIGHTMAP);
        assert_eq!(nb_steps(&heightmap, Path::StartToEnd), 31);
    }

    #[test]
    fn part2() {
        let heightmap = parse(HEIGHTMAP);
        assert_eq!(nb_steps(&heightmap, Path::EndTo0Elevation), 29);
    }
}
