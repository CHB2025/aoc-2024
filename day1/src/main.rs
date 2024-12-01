use std::collections::HashMap;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Part one: {}", part_one(INPUT));
    println!("Part two: {}", part_two(INPUT));
}

fn part_one(input: &str) -> i32 {
    let (mut first, mut second) = (Vec::new(), Vec::new());
    for (f, s) in input.lines().map(|s| s.trim().split_once("   ").unwrap()) {
        let f = f.parse::<i32>().unwrap();
        let s = s.parse::<i32>().unwrap();
        let fi = first.binary_search(&f).unwrap_or_else(|e| e);
        let si = second.binary_search(&s).unwrap_or_else(|e| e);
        first.insert(fi, f);
        second.insert(si, s);
    }
    assert!(first.is_sorted());
    assert!(second.is_sorted());

    first
        .into_iter()
        .zip(second)
        .map(|(a, b)| a.abs_diff(b) as i32)
        .sum()
}

fn part_two(input: &str) -> i32 {
    let (mut first, mut second) = (Vec::new(), Vec::new());
    for (f, s) in input.lines().map(|s| s.trim().split_once("   ").unwrap()) {
        first.push(f.parse::<i32>().unwrap());
        second.push(s.parse::<i32>().unwrap());
    }
    let mut map = HashMap::<i32, i32>::new();
    first
        .into_iter()
        .map(|v| {
            v * *map
                .entry(v)
                .or_insert_with(|| second.iter().filter(|&&s| s == v).count() as i32)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_part_one() {
        let expected: i32 = 11;
        assert_eq!(expected, part_one(INPUT))
    }

    #[test]
    fn test_part_two() {
        let expected: i32 = 31;
        assert_eq!(expected, part_two(INPUT))
    }
}
