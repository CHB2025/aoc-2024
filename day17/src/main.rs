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

    pub fn produces_copy(mut self) -> bool {
        while self.pointer < self.program.len() {
            self.step();
            if !self.program.starts_with(&self.output) {
                return false;
            }
        }
        self.program == self.output
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

fn reverse(program: &[u8]) -> Num {
    let (mut a, mut b, mut c) = (0, 0, 0); // Not sure yet how to initialize B and C
    let mut pointer = program.len();
    let mut output = program;
    while !output.is_empty() || pointer > 0 {
        pointer = pointer.checked_sub(2).unwrap_or(program.len() - 4);
        let instruction = program[pointer];
        let arg = program[pointer + 1];
        match instruction {
            0 => a <<= combo_val(arg, (a, b, c)), // adv
            1 => b ^= arg as Num,                 // bxl
            // 2 => b = combo_val(arg, (a, b, c)) & 0b111, // todo
            2 => {
                let bv = b;
                let Some(reg) = combo(arg, (&mut a, &mut b, &mut c)) else {
                    continue;
                };
                *reg = (*reg & !0b111) | (bv & 0b111);
            }
            3 => (), // jnz -noop
            4 => b ^= c,
            5 => {
                let out: &u8;
                (out, output) = output.split_last().unwrap();
                let reg = combo(arg, (&mut a, &mut b, &mut c)).unwrap();
                *reg = (*reg & !0b111) | (*out as Num);
            }
            6 => b = a << combo_val(arg, (a, b, c)), // not sure what to do with these
            7 => c = a << combo_val(arg, (a, b, c)),
            _ => unreachable!(),
        }
    }
    a
}
fn combo<'a>(arg: u8, (a, b, c): (&'a mut Num, &'a mut Num, &'a mut Num)) -> Option<&mut Num> {
    match arg {
        0..=3 => None,
        4 => Some(a),
        5 => Some(b),
        6 => Some(c),
        7 => panic!("Reserved"),
        _ => unreachable!(),
    }
}
fn combo_val(arg: u8, (a, b, c): (Num, Num, Num)) -> Num {
    match arg {
        a @ 0..=3 => a as Num,
        4 => a,
        5 => b,
        6 => c,
        7 => panic!("Reserved"),
        _ => unreachable!(),
    }
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
    let mut a = 0;
    let mut computer = Computer {
        program: &program,
        pointer: 0,
        a,
        b,
        c,
        output: Vec::with_capacity(program.len()),
    };
    while computer.clone().run().len() < program.len() {
        a += 10_000_000;
        computer = Computer {
            program: &program,
            pointer: 0,
            a,
            b,
            c,
            output: Vec::with_capacity(program.len()),
        }
    }
    loop {
        if a % 10_000_000 == 0 {
            println!("{a}\r");
        }
        let computer = Computer {
            program: &program,
            pointer: 0,
            a,
            b,
            c,
            output: Vec::with_capacity(program.len()),
        };
        if computer.produces_copy() {
            return a;
        }
        a += 1;
    }
    // reverse(&program)
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
