use std::cmp::{Ord, Ordering, PartialOrd};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Signal {
    Value(i32),
    List(Vec<Signal>),
}

impl Signal {
    fn parse(s: &str) -> Self {
        fn parse_chars<'a>(chars: &mut impl Iterator<Item = &'a [char]>) -> Signal {
            let mut n = String::new();
            let mut l: Vec<Signal> = Vec::new();
            while let Some([c1, c2]) = chars.next() {
                match c1 {
                    '[' => {
                        if *c2 == ']' {
                            chars.next();
                            return Signal::List(l);
                        }
                        l.push(parse_chars(chars));
                    }
                    ']' => return Signal::List(l),
                    ',' => l.push(parse_chars(chars)),
                    _ if c1.is_digit(10) => {
                        n.push(*c1);
                        if !c2.is_digit(10) {
                            return Signal::Value(n.parse().unwrap());
                        }
                    }
                    _ => (),
                }
            }
            Signal::Value(n.parse().unwrap())
        }
        let mut chars = s.replace(" ", "").chars().collect::<Vec<char>>();
        // Add a space because only the first character of 'windows(2)' is processed.
        chars.push(' ');
        parse_chars(&mut chars.windows(2))
    }
}

impl Ord for Signal {
    fn cmp(&self, other: &Signal) -> Ordering {
        match (self, other) {
            (Signal::Value(v1), Signal::Value(v2)) => v1.cmp(v2),
            (v1 @ Signal::Value(_), l2 @ Signal::List(_)) => Signal::List(vec![v1.clone()]).cmp(l2),
            (l1 @ Signal::List(_), v2 @ Signal::Value(_)) => {
                l1.cmp(&Signal::List(vec![v2.clone()]))
            }
            (Signal::List(l1), Signal::List(l2)) => {
                for i in 0..l1.len().min(l2.len()) {
                    match l1[i].cmp(&l2[i]) {
                        Ordering::Equal => (),
                        other => return other,
                    }
                }
                l1.len().cmp(&l2.len())
            }
        }
    }
}

impl PartialOrd for Signal {
    fn partial_cmp(&self, other: &Signal) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn parse(input: &str) -> Vec<Signal> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(Signal::parse)
        .collect()
}

pub fn sum_indices_signals_in_the_right_order(signals: &[Signal]) -> usize {
    signals.chunks(2).enumerate().fold(0, |sum, (n, chunk)| {
        if chunk[0] < chunk[1] {
            sum + n + 1
        } else {
            sum
        }
    })
}

pub fn product_indices_special_signals(signals: &[Signal]) -> usize {
    let mut signals = Vec::from(signals);
    let s1 = Signal::parse("[[2]]");
    let s2 = Signal::parse("[[6]]");
    signals.push(s1.clone());
    signals.push(s2.clone());
    signals.sort();
    (signals.binary_search(&s1).unwrap() + 1) * (signals.binary_search(&s2).unwrap() + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_tests() {
        assert_eq!(Signal::parse("[]"), Signal::List(Vec::new()));
        assert_eq!(Signal::parse("1"), Signal::Value(1));
        assert_eq!(Signal::parse("123"), Signal::Value(123));
        assert_eq!(Signal::parse("[1]"), Signal::List(vec![Signal::Value(1)]));
        assert_eq!(
            Signal::parse("[1,[]]"),
            Signal::List(vec![Signal::Value(1), Signal::List(Vec::new())])
        );
        assert_eq!(
            Signal::parse("[[], 1]"),
            Signal::List(vec![Signal::List(Vec::new()), Signal::Value(1)])
        );
        assert_eq!(
            Signal::parse(" [   1,[  [ ] ,2  ]   ]  "),
            Signal::List(vec![
                Signal::Value(1),
                Signal::List(vec![Signal::List(Vec::new()), Signal::Value(2)])
            ])
        );
    }

    #[test]
    fn comparison() {
        assert!(Signal::parse("[1,1,3,1,1]")
            .cmp(&Signal::parse("[1,1,5,1,1]"))
            .is_lt());
        assert!(Signal::parse("[[1],[2,3,4]]")
            .cmp(&Signal::parse("[[1],4]"))
            .is_lt());
        assert!(Signal::parse("[9]")
            .cmp(&Signal::parse("[[8,7,6]]"))
            .is_gt());
        assert!(Signal::parse("[[4,4],4,4]")
            .cmp(&Signal::parse("[[4,4],4,4,4]"))
            .is_lt());
        assert!(Signal::parse("[7,7,7,7]")
            .cmp(&Signal::parse("[7,7,7]"))
            .is_gt());
        assert!(Signal::parse("[]").cmp(&Signal::parse("[3]")).is_lt());
        assert!(Signal::parse("[[[]]]").cmp(&Signal::parse("[[]]")).is_gt());
        assert!(Signal::parse("[1,[2,[3,[4,[5,6,7]]]],8,9]")
            .cmp(&Signal::parse("[1,[2,[3,[4,[5,6,0]]]],8,9]"))
            .is_gt());
    }

    static SIGNALS: &str = "[1,1,3,1,1]
        [1,1,5,1,1]

        [[1],[2,3,4]]
        [[1],4]

        [9]
        [[8,7,6]]

        [[4,4],4,4]
        [[4,4],4,4,4]

        [7,7,7,7]
        [7,7,7]

        []
        [3]

        [[[]]]
        [[]]

        [1,[2,[3,[4,[5,6,7]]]],8,9]
        [1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn part1() {
        let signals = parse(SIGNALS);
        assert_eq!(sum_indices_signals_in_the_right_order(&signals), 13);
    }

    #[test]
    fn part2() {
        let signals = parse(SIGNALS);
        assert_eq!(product_indices_special_signals(&signals), 140);
    }
}
