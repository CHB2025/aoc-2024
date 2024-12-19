const INPUT: &str = include_str!("../input.txt");

type Num = usize;

#[derive(Clone)]
struct Computer<'p> {
    program: &'p [u8],
    pointer: usize,
    a: Num,
    b: Num,
    c: Num,
    output: Vec<u8>,
}

impl<'p> Computer<'p> {
    pub fn run(mut self) -> Vec<u8> {
        while self.pointer < self.program.len() {
            self.step();
        }
        self.output
    }

    fn step(&mut self) {
        let instruction = self.program[self.pointer];
        let arg = self.program[self.pointer + 1];
        self.pointer += 2;
        match instruction {
            0 => self.a >>= self.combo(arg),       // adv
            1 => self.b ^= arg as Num,             // bxl
            2 => self.b = self.combo(arg) & 0b111, // bst
            3 => {
                // jnz
                if self.a != 0 {
                    self.pointer = arg as Num
                }
            }
            4 => self.b ^= self.c,
            5 => self.output.push((self.combo(arg) & 0b0111) as u8),
            6 => self.b = self.a >> self.combo(arg),
            7 => self.c = self.a >> self.combo(arg),
            _ => unreachable!(),
        }
    }

    fn combo(&self, arg: u8) -> Num {
        match arg {
            a @ 0..=3 => a as Num,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            7 => panic!("Reserved"),
            _ => unreachable!(),
        }
    }
}

fn main() {
    println!("Part one: {}", part_one(INPUT));
    println!("Part two: {}", part_two(INPUT));
}

fn part_one(input: &str) -> String {
    let mut lines = input.lines();
    let a = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .parse::<Num>()
        .unwrap();
    let b = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .parse::<Num>()
        .unwrap();
    let c = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .parse::<Num>()
        .unwrap();

    _ = lines.next();
    let program = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .split(',')
        .map(|n| n.parse::<u8>().unwrap())
        .collect::<Vec<_>>();
    let computer = Computer {
        program: &program,
        pointer: 0,
        a,
        b,
        c,
        output: Vec::new(),
    };
    computer.run().into_iter().fold(String::new(), |str, o| {
        if str.is_empty() {
            format!("{o}")
        } else {
            format!("{str},{o}")
        }
    })
}

fn part_two(input: &str) -> Num {
    // likely need to solve rather than check all numbers Programs all seem
    // to end in "3,0" and have no other jumps, so we know A = 0 at the end.
    // However, we don't know the state of B and C at the end of the program, so
    // we can't just reverse it Also, some instructions (like adv, bdv, and cdv)
    // are irreversible?
    let mut lines = input.lines();
    _ = lines.next(); // Ignore A
    let b = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .parse::<Num>()
        .unwrap();
    let c = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .parse::<Num>()
        .unwrap();

    _ = lines.next();
    let program = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .split(',')
        .map(|n| n.parse::<u8>().unwrap())
        .collect::<Vec<_>>();
    let a = solve(0, (0, b, c), &program).unwrap();
    let comp = Computer {
        program: &program,
        pointer: 0,
        a,
        b,
        c,
        output: Vec::new(),
    }
    .run();
    assert_eq!(comp, program);
    a
}

// build incrementally, 3 bits at a time
// Help from: https://www.reddit.com/r/adventofcode/comments/1hg38ah/comment/m2pyn7q/
fn solve(i: usize, (a, b, c): (Num, Num, Num), program: &[u8]) -> Option<Num> {
    let output = Computer {
        program,
        pointer: 0,
        a,
        b,
        c,
        output: Vec::new(),
    }
    .run();
    if output == program {
        return Some(a);
    }
    if program[program.len() - i..] == output || i == 0 {
        for n in 0..8usize {
            if let Some(a) = solve(i + 1, ((a << 3) + n, b, c), program) {
                return Some(a);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    #[test]
    fn test_part_one() {
        let expected: &str = "4,6,3,5,6,3,5,2,1,0";
        assert_eq!(expected, part_one(INPUT))
    }

    #[test]
    fn test_part_two() {
        let expected: Num = 117440;
        assert_eq!(
            expected,
            part_two(
                "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0"
            )
        )
    }
}
