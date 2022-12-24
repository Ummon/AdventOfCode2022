pub struct Cube {
    x: i32,
    y: i32,
    z: i32,
}

pub fn parse(input: &str) -> Vec<Cube> {
    input
        .lines()
        .map(|l| {
            let xyz: Vec<i32> = l
                .trim()
                .split(',')
                .map(|v| str::parse(v).unwrap())
                .collect();
            Cube {
                x: xyz[0],
                y: xyz[1],
                z: xyz[2],
            }
        })
        .collect()
}

pub fn surface(cubes: &[Cube]) -> i32 {
    let mut matrix: Vec<Vec<Vec<bool>>> = Vec::new();

    for c in cubes {
        for _ in matrix.len()..=c.x as usize {
            matrix.push(Vec::new())
        }
        for _ in matrix[c.x as usize].len()..=c.y as usize {
            matrix[c.x as usize].push(Vec::new())
        }
        for _ in matrix[c.x as usize][c.y as usize].len()..=c.z as usize {
            matrix[c.x as usize][c.y as usize].push(false)
        }
        matrix[c.x as usize][c.y as usize][c.z as usize] = true;
    }

    let mut surface: i32 = 0;
    for x in 0..matrix.len() as i32 {
        for y in 0..matrix[x as usize].len() as i32 {
            for z in 0..matrix[x as usize][y as usize].len() as i32 {
                if matrix[x as usize][y as usize][z as usize] {
                    for (dx, dy, dz) in [
                        (1, 0, 0),
                        (-1, 0, 0),
                        (0, 1, 0),
                        (0, -1, 0),
                        (0, 0, 1),
                        (0, 0, -1),
                    ] {
                        let (x, y, z) = (x + dx, y + dy, z + dz);
                        if x < 0
                            || x >= matrix.len() as i32
                            || y < 0
                            || y >= matrix[x as usize].len() as i32
                            || z < 0
                            || z >= matrix[x as usize][y as usize].len() as i32
                            || !matrix[x as usize][y as usize][z as usize]
                        {
                            surface = surface + 1;
                        }
                    }
                }
            }
        }
    }

    surface
}

pub fn surface_without_trapped_air(cubes: &[Cube]) -> i32 {
    0 // TODO
}

#[cfg(test)]
mod tests {
    use super::*;

    static CUBES: &str = "2,2,2
    1,2,2
    3,2,2
    2,1,2
    2,3,2
    2,2,1
    2,2,3
    2,2,4
    2,2,6
    1,2,5
    3,2,5
    2,1,5
    2,3,5";

    #[test]
    fn part1() {
        let cubes = parse(CUBES);
        assert_eq!(surface(&cubes), 64);
    }

    #[test]
    fn part2() {}
}
