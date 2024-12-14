use std::{error::Error, str::FromStr};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Part one: {}", part_one(INPUT));
    println!("Part two: {}", part_two(INPUT));
}

#[derive(Debug, Clone)]
struct ArcadeMachine {
    pub a: (usize, usize),
    pub b: (usize, usize),
    pub prize: (usize, usize),
}

impl ArcadeMachine {
    // Only ever 1, unless a = b * n (which doesn't happen in my input)
    // pub fn path(&self) -> Option<(usize, usize)> {
    //     for a in 0.. {
    //         if self.a.0 * a > self.prize.0 || self.a.1 * a > self.prize.1 {
    //             break;
    //         }
    //         let rem = (self.prize.0 - a * self.a.0, self.prize.1 - a * self.a.1);
    //         if rem.0 % self.b.0 == 0
    //             && rem.1 % self.b.1 == 0
    //             && rem.0 / self.b.0 == rem.1 / self.b.1
    //         {
    //             return Some((a, rem.0 / self.b.0));
    //         }
    //     }
    //     None
    // }

    // Algrebra!
    pub fn path(&self) -> Option<(usize, usize)> {
        let b_dividend = (self.prize.1 * self.a.0) as i64 - (self.prize.0 * self.a.1) as i64;
        let b_divisor = (self.a.0 * self.b.1) as i64 - (self.b.0 * self.a.1) as i64;
        if b_dividend % b_divisor != 0 {
            return None;
        }
        let b_presses = b_dividend / b_divisor;

        let a_dividend = self.prize.0 as i64 - b_presses * self.b.0 as i64;
        if a_dividend % self.a.0 as i64 != 0 {
            return None;
        }
        let a_presses = a_dividend / self.a.0 as i64;

        if a_presses.is_negative() || b_presses.is_negative() {
            return None;
        }
        Some((a_presses as usize, b_presses as usize))
    }
}

impl FromStr for ArcadeMachine {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let button_a = lines.next().ok_or("Missing line for Button A")?;
        let button_b = lines.next().ok_or("Missing line for Button B")?;
        let prize_line = lines.next().ok_or("Missing line for prize")?;

        Ok(Self {
            a: parse_button(button_a)?,
            b: parse_button(button_b)?,
            prize: parse_button(prize_line)?,
        })
    }
}

fn parse_button(s: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let xs = s.find('X').ok_or("Button missing X coord")? + 2;
    let xe = s.find(',').ok_or("Button missing end of X coord")?;
    let x = s[xs..xe].parse::<usize>()?;
    let ys = s.find('Y').ok_or("Button missing Y coord")? + 2;
    let y = s[ys..].parse::<usize>()?;
    Ok((x, y))
}

fn part_one(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|s| ArcadeMachine::from_str(s).unwrap())
        .filter_map(|a| a.path())
        .map(|(ac, bc)| ac * 3 + bc)
        .sum()
}

fn part_two(input: &str) -> usize {
    const ERROR: usize = 10_000_000_000_000;
    input
        .split("\n\n")
        .map(|s| {
            let mut am = ArcadeMachine::from_str(s).unwrap();
            am.prize.0 += ERROR;
            am.prize.1 += ERROR;
            am
        })
        .filter_map(|a| a.path())
        .map(|(ac, bc)| ac * 3 + bc)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn test_part_one() {
        let expected: usize = 480;
        assert_eq!(expected, part_one(INPUT))
    }

    #[test]
    fn test_part_two() {
        let expected: usize = 875318608908;
        assert_eq!(expected, part_two(INPUT))
    }
}
