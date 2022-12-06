pub fn first_marker_pos(signal: &str, n: usize) -> usize {
    for (i, c) in signal.chars().collect::<Vec<char>>().windows(n).enumerate() {
        if !contains_same_char(c) {
            return i + n;
        }
    }
    0
}

// Warning: O(n^2).
fn contains_same_char(chars: &[char]) -> bool {
    for i in 0..chars.len() {
        for j in i+1..chars.len() {
            if chars[i] == chars[j] {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(first_marker_pos("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4), 7);
        assert_eq!(first_marker_pos("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
        assert_eq!(first_marker_pos("nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
        assert_eq!(first_marker_pos("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4), 10);
        assert_eq!(first_marker_pos("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4), 11);
    }

    #[test]
    fn part2() {
        assert_eq!(first_marker_pos("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14), 19);
        assert_eq!(first_marker_pos("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), 23);
        assert_eq!(first_marker_pos("nppdvjthqldpwncqszvftbrmjlhg", 14), 23);
        assert_eq!(first_marker_pos("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14), 29);
        assert_eq!(first_marker_pos("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14), 26);
    }
}