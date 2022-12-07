use regex::Regex;

type Pair = ((i32, i32), (i32, i32));

pub fn parse(s: &str) -> Vec<Pair> {
    let mut sections = Vec::new();
    let r = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();
    for l in s.lines() {
        let cap = r.captures(l).unwrap();
        sections.push(((cap[1].parse::<i32>().unwrap(), cap[2].parse::<i32>().unwrap()), (cap[3].parse::<i32>().unwrap(), cap[4].parse::<i32>().unwrap())));
    }
    return sections;
}

pub fn number_fully_contain(pairs: &[Pair]) -> i32 {
    pairs.iter().filter_map(|((a1, a2), (b1, b2))| {
        if a1 <= b1 && a2 >= b2 || b1 <= a1 && b2 >= a2 { Some(1) } else { None }
    }).sum()
}

pub fn number_overlaps(pairs: &[Pair]) -> i32 {
    pairs.iter().filter_map(|((a1, a2), (b1, b2))| {
        if a2 >= b1 && a1 <= b2 { Some(1) } else { None }
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static PAIRS: &str ="2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn part1() {
        assert_eq!(number_fully_contain(&parse(PAIRS)), 2);
    }

    #[test]
    fn part2() {
        assert_eq!(number_overlaps(&parse(PAIRS)), 4);
    }
}