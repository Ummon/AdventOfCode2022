use std::str::Lines;

#[derive(Debug)]
pub struct Dir {
    files: Vec<i64>,
    dirs: Vec<Dir>,
}

impl Dir {
    fn new() -> Dir {
        Dir {
            files: Vec::new(),
            dirs: Vec::new(),
        }
    }

    fn get_file_size(&self) -> i64 {
        self.files.iter().sum()
    }

    pub fn dir_sizes<P>(&self, predicate: P, result: &mut Vec<i64>) -> i64
    where
        P: Fn(i64) -> bool + Copy,
    {
        let size = self.get_file_size()
            + self
                .dirs
                .iter()
                .map(|dir| dir.dir_sizes(predicate, result))
                .sum::<i64>();

        if predicate(size) {
            result.push(size)
        }
        size
    }
}

pub fn parse(input: &str) -> Dir {
    fn create_dir(lines: &mut Lines) -> Dir {
        let mut dir = Dir::new();
        while let Some(l) = lines.next() {
            let l: Vec<&str> = l.split(' ').collect();
            if l[0] == "$" {
                if l[1] == "cd" {
                    if l[2] == ".." {
                        return dir;
                    } else {
                        let child = create_dir(lines);
                        dir.dirs.push(child);
                    }
                }
            } else if l[0] != "dir" {
                dir.files.push(l[0].parse().unwrap());
            }
        }
        dir
    }
    let mut lines = input.lines();
    lines.next(); // First line is always the root.
    create_dir(&mut lines)
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn part1() {
        let root = parse(INPUT);
        let mut sizes: Vec<i64> = Vec::new();
        root.dir_sizes(|size| size <= 100_000, &mut sizes);
        assert_eq!(sizes.iter().sum::<i64>(), 95_437);
    }

    #[test]
    fn part2() {
        let root = parse(INPUT);
        let root_size = root.dir_sizes(|size| size <= 100_000, &mut Vec::new());
        let to_free = root_size - (70_000_000 - 30_000_000);
        let mut sizes: Vec<i64> = Vec::new();
        root.dir_sizes(|size| size >= to_free, &mut sizes);
        assert_eq!(*sizes.iter().min().unwrap(), 2_493_3642);
    }
}
