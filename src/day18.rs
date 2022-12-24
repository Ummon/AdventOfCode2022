pub struct Cube {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Clone, Copy)]
pub enum Element {
    Empty,
    Obsidian,
    Droplet,
}

type Mat3D = Vec<Vec<Vec<Element>>>;

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

static NEIGHBOURS: [(i32, i32, i32); 6] = [
    (1, 0, 0),
    (-1, 0, 0),
    (0, 1, 0),
    (0, -1, 0),
    (0, 0, 1),
    (0, 0, -1),
];

pub fn surface(cubes: &[Cube]) -> (i32, Mat3D) {
    let mut matrix: Mat3D = Vec::new();

    for c in cubes {
        for _ in matrix.len()..=c.x as usize {
            matrix.push(Vec::new())
        }
        for _ in matrix[c.x as usize].len()..=c.y as usize {
            matrix[c.x as usize].push(Vec::new())
        }
        for _ in matrix[c.x as usize][c.y as usize].len()..=c.z as usize {
            matrix[c.x as usize][c.y as usize].push(Element::Empty)
        }
        matrix[c.x as usize][c.y as usize][c.z as usize] = Element::Obsidian;
    }

    let mut surface: i32 = 0;
    for x in 0..matrix.len() as i32 {
        for y in 0..matrix[x as usize].len() as i32 {
            for z in 0..matrix[x as usize][y as usize].len() as i32 {
                if let Element::Obsidian = matrix[x as usize][y as usize][z as usize] {
                    for (dx, dy, dz) in NEIGHBOURS {
                        let (x, y, z) = (x + dx, y + dy, z + dz);
                        if x < 0
                            || x >= matrix.len() as i32
                            || y < 0
                            || y >= matrix[x as usize].len() as i32
                            || z < 0
                            || z >= matrix[x as usize][y as usize].len() as i32
                            || matches!(matrix[x as usize][y as usize][z as usize], Element::Empty)
                        {
                            surface += 1;
                        }
                    }
                }
            }
        }
    }

    (surface, matrix)
}

enum FloodResult {
    TouchingLimits,
    InnerSurface(i32),
}

fn flood(m: &mut Mat3D, x: i32, y: i32, z: i32) -> FloodResult {
    let mut to_visit = vec![(x, y, z)];
    let mut surface = 0;
    let mut touching_limits = false;

    while let Some((x, y, z)) = to_visit.pop() {
        if let Element::Droplet = m[x as usize][y as usize][z as usize] {
            continue;
        }

        m[x as usize][y as usize][z as usize] = Element::Droplet;
        for (dx, dy, dz) in NEIGHBOURS {
            let (x, y, z) = (x + dx, y + dy, z + dz);
            if x < 0
                || x >= m.len() as i32
                || y < 0
                || y >= m[x as usize].len() as i32
                || z < 0
                || z >= m[x as usize][y as usize].len() as i32
            {
                touching_limits = true;
                continue;
            }
            match m[x as usize][y as usize][z as usize] {
                Element::Empty => to_visit.push((x, y, z)),
                Element::Obsidian => surface += 1,
                Element::Droplet => (),
            }
        }
    }

    if touching_limits {
        FloodResult::TouchingLimits
    } else {
        FloodResult::InnerSurface(surface)
    }
}

pub fn surface_without_trapped_air(outer_surface: i32, mut m: Mat3D) -> i32 {
    let mut inner_surface = 0;
    for x in 0..m.len() {
        for y in 0..m[x].len() {
            for z in 0..m[x][y].len() {
                if let Element::Empty = m[x][y][z] {
                    if let FloodResult::InnerSurface(s) =
                        flood(&mut m, x as i32, y as i32, z as i32)
                    {
                        inner_surface += s;
                    }
                }
            }
        }
    }

    outer_surface - inner_surface
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
        let (surface, _) = surface(&cubes);
        assert_eq!(surface, 64);
    }

    #[test]
    fn part2() {
        let cubes = parse(CUBES);
        let (surface, obsidian) = surface(&cubes);
        assert_eq!(surface_without_trapped_air(surface, obsidian), 58);
    }
}
