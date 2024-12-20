use std::{cmp::Reverse, collections::HashMap};

const INPUT: &str = include_str!("../input.txt");

type Num = usize;

fn main() {
    println!("Part one: {}", part_one(INPUT));
    println!("Part two: {}", part_two(INPUT));
}

fn part_one(input: &str) -> Num {
    let (towels, patterns) = input.split_once("\n\n").unwrap();
    let mut towels = towels.split(',').map(|t| t.trim()).collect::<Vec<_>>();
    towels.sort_by_key(|t| Reverse(t.len()));
    patterns.lines().filter(|p| possible(p, &towels)).count()
}

fn possible(pattern: &str, towels: &[&str]) -> bool {
    if pattern == "" {
        return true;
    }
    for t in towels {
        if pattern.starts_with(t) && possible(&pattern[t.len()..], towels) {
            return true;
        }
    }
    false
}

fn part_two(input: &str) -> Num {
    let (towels, patterns) = input.split_once("\n\n").unwrap();
    let mut towels = towels.split(',').map(|t| t.trim()).collect::<Vec<_>>();
    towels.sort_by_key(|t| Reverse(t.len()));
    let mut cache = HashMap::new(); // global since towels are the same
    patterns
        .lines()
        .map(|p| count_possiblities(p, &towels, &mut cache))
        .sum()
}

fn count_possiblities<'a>(
    pattern: &'a str,
    towels: &[&str],
    cache: &mut HashMap<&'a str, usize>,
) -> usize {
    if pattern == "" {
        return 1; // really caller is 1: t + ""
    }
    if let Some(count) = cache.get(pattern) {
        return *count;
    }
    let count = {
        towels
            .iter()
            .filter(|t| pattern.starts_with(**t))
            .map(|t| count_possiblities(&pattern[t.len()..], towels, cache))
            .sum()
    };
    cache.insert(pattern, count);
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    #[test]
    fn test_part_one() {
        let expected: Num = 6;
        assert_eq!(expected, part_one(INPUT))
    }

    #[test]
    fn test_part_two() {
        let expected: Num = 16;
        assert_eq!(expected, part_two(INPUT))
    }
}
