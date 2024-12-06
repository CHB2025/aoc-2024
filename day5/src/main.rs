use std::{cmp::Ordering, collections::HashMap};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Part one: {}", part_one(INPUT));
    println!("Part two: {}", part_two(INPUT));
}

fn contain_same_elements<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    a.iter().any(|n| b.contains(n))
}

// when we find number n, check all the numbers that must be after and see if
// we've seen them before. If we have, the line is invalid
fn part_one(input: &str) -> usize {
    let (rules, pages) = input.split_once("\n\n").unwrap();
    let rules = rules
        .lines()
        .map(|r| {
            let (a, b) = r.split_once('|').unwrap();
            (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap())
        })
        .fold(HashMap::<usize, Vec<usize>>::new(), |mut map, (k, v)| {
            map.entry(k).or_default().push(v);
            map
        });
    pages
        .lines()
        .filter_map(|l| {
            l.split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .try_fold(Vec::new(), |mut v, n| {
                    if rules.get(&n).is_some_and(|s| contain_same_elements(s, &v)) {
                        None
                    } else {
                        v.push(n);
                        Some(v)
                    }
                })
        })
        .map(|update| update[update.len() / 2])
        .sum()
}

fn compare(map: &HashMap<usize, Vec<usize>>, a: usize, b: usize) -> Ordering {
    if map.get(&a).is_some_and(|r| r.contains(&b)) {
        Ordering::Less
    } else if map.get(&b).is_some_and(|r| r.contains(&a)) {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

fn part_two(input: &str) -> usize {
    let (rules, pages) = input.split_once("\n\n").unwrap();
    let rules = rules
        .lines()
        .map(|r| {
            let (a, b) = r.split_once('|').unwrap();
            (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap())
        })
        .fold(HashMap::<usize, Vec<usize>>::new(), |mut map, (k, v)| {
            map.entry(k).or_default().push(v);
            map
        });
    pages
        .lines()
        .map(|l| {
            // Harder to do a fold here
            l.split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|update| {
            !update.iter().enumerate().all(|(i, n)| {
                !rules
                    .get(n)
                    .is_some_and(|s| contain_same_elements(s, &update[..i]))
            })
        })
        .map(|mut v| {
            v.sort_by(|a, b| compare(&rules, *a, *b));
            v[v.len() / 2]
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_part_one() {
        let expected: usize = 143;
        assert_eq!(expected, part_one(INPUT))
    }

    #[test]
    fn test_part_two() {
        let expected: usize = 123;
        assert_eq!(expected, part_two(INPUT))
    }
}
