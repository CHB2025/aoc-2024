use std::{
    collections::{HashMap, HashSet},
    convert::Infallible,
    str::FromStr,
};

const INPUT: &str = include_str!("../input.txt");

type Num = usize;

struct Keypad {
    precomputed: HashMap<(char, char), Vec<String>>,
    cache: HashMap<String, Vec<usize>>, // Vec instead of (String, usize) so we can query with &str
}

impl Keypad {
    pub fn expand(&self, seq: &str) -> Vec<String> {
        let mut position = 'A';
        let mut output = vec![String::new()];
        for c in seq.chars() {
            let paths = self.precomputed.get(&(position, c)).unwrap();
            output = output
                .into_iter()
                .flat_map(|p| {
                    paths.iter().map(move |ext| {
                        let mut o = p.clone();
                        o.push_str(ext);
                        o
                    })
                })
                .collect();
            position = c;
        }
        output
    }

    // Cost in movements to arrive to a sequence with depth d
    pub fn cost(&mut self, seq: String, d: usize) -> usize {
        if d == 0 {
            return seq.len();
        }
        if let Some(c) = self
            .cache
            .get(&seq)
            .and_then(|v| v.get(d - 1).filter(|&&v| v != 0))
        {
            return *c;
        }
        let mut position = 'A';
        let mut cost = 0;
        for c in seq.chars() {
            let options = self.precomputed.get(&(position, c)).unwrap().clone();
            cost += options
                .into_iter()
                .map(|seq| self.cost(seq, d - 1))
                .min()
                .unwrap();
            position = c;
        }
        let vec = self.cache.entry(seq).or_insert(Vec::new());
        if vec.len() < d {
            vec.resize(d, 0);
        }
        vec.insert(d - 1, cost);
        cost
    }
}

// to minimize presses, must first do all one direction, then all the other.
// prioritize <, then v, then ^ then >
fn find(
    map: &HashSet<(usize, usize)>,
    start: (usize, usize),
    target: (usize, usize),
) -> Vec<String> {
    let x_keys = if start.0 > target.0 { "<" } else { ">" }.repeat(start.0.abs_diff(target.0));
    let y_keys = if start.1 > target.1 { "^" } else { "v" }.repeat(start.1.abs_diff(target.1));
    if y_keys.is_empty() {
        return vec![format!("{x_keys}A")];
    }
    if x_keys.is_empty() {
        return vec![format!("{y_keys}A")];
    }

    let mut out = Vec::new();
    if map.contains(&(target.0, start.1)) {
        out.push(format!("{x_keys}{y_keys}A"));
    }
    if map.contains(&(start.0, target.1)) {
        out.push(format!("{y_keys}{x_keys}A"));
    }
    assert!(!out.is_empty());
    out
}

impl FromStr for Keypad {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let keys: HashMap<_, _> = s
            .lines()
            .enumerate()
            .flat_map(|(y, l)| l.chars().enumerate().map(move |(x, c)| (c, (x, y))))
            .filter(|(c, _)| *c != ' ')
            .collect();
        let map = keys.values().cloned().collect();
        let precomputed = keys
            .iter()
            .flat_map(|(&s, &start)| {
                let map = &map;
                keys.iter()
                    .map(move |(&e, &end)| ((s, e), find(map, start, end)))
            })
            .collect();
        Ok(Self {
            precomputed,
            cache: HashMap::new(),
        })
    }
}

fn main() {
    println!("Part one: {}", part_one(INPUT));
    println!("Part two: {}", part_two(INPUT));
}

fn part_one(input: &str) -> Num {
    let keypad = Keypad::from_str("789\n456\n123\n 0A").unwrap();
    let mut robot_keypad = Keypad::from_str(" ^A\n<v>").unwrap();

    input
        .lines()
        .map(|l| (l, keypad.expand(&l)))
        .map(|(p, v)| {
            (
                p[..p.len() - 1].parse::<usize>().unwrap(),
                v.into_iter()
                    .map(|seq| robot_keypad.cost(seq, 2))
                    .min()
                    .unwrap(),
            )
        })
        .map(|(p, l)| p * l)
        .inspect(|c| println!("\t{c}"))
        .sum()
}

fn part_two(input: &'static str) -> Num {
    let keypad = Keypad::from_str("789\n456\n123\n 0A").unwrap();
    let mut robot_keypad = Keypad::from_str(" ^A\n<v>").unwrap();

    input
        .lines()
        .map(|l| (l, keypad.expand(&l)))
        .map(|(p, v)| {
            println!("{p}");
            (
                p[..p.len() - 1].parse::<usize>().unwrap(),
                v.into_iter()
                    .inspect(|seq| println!("\t{seq}"))
                    .map(|seq| robot_keypad.cost(seq, 25))
                    .min()
                    .unwrap(),
            )
        })
        .map(|(p, l)| p * l)
        .inspect(|c| println!("\t{c}"))
        .sum() // 170189566432 wrong
               // 749743435601420 wrong. Seems way too high
               // 133644090418250 wrong.
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "029A
980A
179A
456A
379A";

    #[test]
    fn test_part_one() {
        let expected: Num = 126384;
        assert_eq!(expected, part_one(INPUT))
    }
}
