use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../input.txt");

type Num = usize;

fn main() {
    println!("Part one: {}", part_one(INPUT));
    println!("Part two: {}", part_two(INPUT));
}

fn next_secret(mut secret: usize) -> usize {
    const PRUNE: usize = 16777216;
    secret ^= secret * 64;
    secret %= PRUNE;
    secret ^= secret / 32;
    secret %= PRUNE;
    secret ^= secret * 2048;
    secret %= PRUNE;
    secret
}

fn price(secret: usize) -> i8 {
    (secret % 10) as i8
}

fn part_one(input: &str) -> Num {
    input
        .lines()
        .map(|l| (l, l.parse::<Num>().unwrap()))
        .map(|(l, mut s)| {
            (0..2000).for_each(|_| s = next_secret(s));
            (l, s)
        })
        .map(|(_, s)| s)
        .sum()
}

fn part_two(input: &str) -> Num {
    let changes = input
        .lines()
        .map(|l| l.parse::<Num>().unwrap())
        .map(|mut s| {
            let mut prices = Vec::new();
            let mut diffs = Vec::new();
            let mut prev = price(s);
            for _ in 0..2000 {
                s = next_secret(s);
                let price = price(s);
                prices.push(price);
                diffs.push(price - prev);
                prev = price;
            }
            (prices, diffs)
        })
        .flat_map(|(pv, dv)| {
            let mut filter = HashSet::new();
            dv.windows(4)
                .zip(pv.into_iter().skip(3))
                .map(|(seq, price)| (seq.try_into().unwrap(), price as usize))
                .filter(|(seq, _)| filter.insert(*seq))
                .collect::<Vec<_>>()
        })
        .fold(HashMap::<[i8; 4], usize>::new(), |mut m, (k, v)| {
            *m.entry(k).or_insert(0usize) += v;
            m
        });
    changes.into_iter().max_by_key(|(_, v)| *v).unwrap().1
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1
10
100
2024";

    #[test]
    fn test_part_one() {
        let expected: Num = 37_327_623;
        assert_eq!(expected, part_one(INPUT))
    }

    #[test]
    fn test_part_two() {
        let expected: Num = 23;
        assert_eq!(
            expected,
            part_two(
                "1
2
3
2024"
            )
        )
    }
}
