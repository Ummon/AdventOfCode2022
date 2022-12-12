pub enum Instruction {
    Noop,
    Addx(i32),
}

pub fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|l| {
            let split: Vec<&str> = l.trim().split(' ').collect();
            match split[0] {
                "noop" => Instruction::Noop,
                "addx" => Instruction::Addx(split[1].parse::<i32>().unwrap()),
                other => panic!("Unknown instruction: {}", other),
            }
        })
        .collect()
}

pub struct Screen {
    screen: Vec<Vec<bool>>,
}

impl Screen {
    pub fn new() -> Self {
        Screen { screen: Vec::new() }
    }

    pub fn draw_screen(&mut self, instructions: &[Instruction]) -> i32 {
        let mut x = 1; // Middle sprite position, sprite is 3 pixels wide.
        let mut signal_strength = 0;
        let mut cycle = 0;

        let mut tick = |x: &i32| {
            let pos_x = cycle % 40;
            let pos_y = cycle / 40;
            if pos_x == 0 {
                self.screen.push(vec![false; 40])
            }

            if pos_x >= x - 1 && pos_x <= x + 1 {
                self.screen[pos_y as usize][pos_x as usize] = true;
            }

            cycle += 1;

            if (cycle - 20) % 40 == 0 {
                signal_strength += x * cycle;
            }
        };

        for i in instructions {
            match i {
                Instruction::Noop => tick(&x),
                Instruction::Addx(v) => {
                    tick(&x);
                    tick(&x);
                    x += *v;
                }
            }
        }

        signal_strength
    }

    pub fn to_ascii(&self) -> String {
        let mut ascii = String::new();
        for line in self.screen.iter() {
            ascii += &line
                .iter()
                .map(|p| if *p { '#' } else { '.' })
                .collect::<String>();
            ascii += "\n";
        }
        ascii
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INSTRUCTION: &str = "addx 15
        addx -11
        addx 6
        addx -3
        addx 5
        addx -1
        addx -8
        addx 13
        addx 4
        noop
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx -35
        addx 1
        addx 24
        addx -19
        addx 1
        addx 16
        addx -11
        noop
        noop
        addx 21
        addx -15
        noop
        noop
        addx -3
        addx 9
        addx 1
        addx -3
        addx 8
        addx 1
        addx 5
        noop
        noop
        noop
        noop
        noop
        addx -36
        noop
        addx 1
        addx 7
        noop
        noop
        noop
        addx 2
        addx 6
        noop
        noop
        noop
        noop
        noop
        addx 1
        noop
        noop
        addx 7
        addx 1
        noop
        addx -13
        addx 13
        addx 7
        noop
        addx 1
        addx -33
        noop
        noop
        noop
        addx 2
        noop
        noop
        noop
        addx 8
        noop
        addx -1
        addx 2
        addx 1
        noop
        addx 17
        addx -9
        addx 1
        addx 1
        addx -3
        addx 11
        noop
        noop
        addx 1
        noop
        addx 1
        noop
        noop
        addx -13
        addx -19
        addx 1
        addx 3
        addx 26
        addx -30
        addx 12
        addx -1
        addx 3
        addx 1
        noop
        noop
        noop
        addx -9
        addx 18
        addx 1
        addx 2
        noop
        noop
        addx 9
        noop
        noop
        noop
        addx -1
        addx 2
        addx -37
        addx 1
        addx 3
        noop
        addx 15
        addx -21
        addx 22
        addx -6
        addx 1
        noop
        addx 2
        addx 1
        noop
        addx -10
        noop
        noop
        addx 20
        addx 1
        addx 2
        addx 2
        addx -6
        addx -11
        noop
        noop
        noop";

    #[test]
    fn part1() {
        let instructions = parse(INSTRUCTION);
        let mut screen = Screen::new();
        assert_eq!(screen.draw_screen(&instructions), 13140);
    }

    #[test]
    fn part2() {
        let instructions = parse(INSTRUCTION);
        let mut screen = Screen::new();
        screen.draw_screen(&instructions);
        println!("{}", screen.to_ascii());

        assert_eq!(
            screen.to_ascii(),
            "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"
        );
    }
}
