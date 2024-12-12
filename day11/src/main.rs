use std::collections::HashMap;

const INPUT: &str = include_str!("../input.txt");

type Num = usize;

fn main() {
    println!("Part one: {}", step_n(INPUT, 25));
    println!("Part two: {}", step_n(INPUT, 75));
}

fn step_n(input: &'static str, n: usize) -> Num {
    let mut cache = HashMap::new();
    input
        .trim()
        .split(' ')
        .map(|n| n.parse::<Num>().unwrap())
        .map(move |s| count(s, n, &mut cache))
        .sum()
}

fn count(n: Num, steps: Num, cache: &mut HashMap<(Num, Num), Num>) -> Num {
    if steps == 0 {
        return 1;
    }
    if let Some(v) = cache.get(&(n, steps)) {
        return *v;
    }
    if n == 0 {
        let c = count(1, steps - 1, cache);
        cache.insert((n, steps), c);
        return c;
    }
    let cd = count_digits(n);
    if cd % 2 == 0 {
        let div = (10 as Num).pow((cd / 2) as u32);
        let c = count(n / div, steps - 1, cache) + count(n % div, steps - 1, cache);
        cache.insert((n, steps), c);
        c
    } else {
        let c = count(n * 2024, steps - 1, cache);
        cache.insert((n, steps), c);
        c
    }
}

fn count_digits(n: Num) -> Num {
    n.ilog10() as Num + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "125 17";

    #[test]
    fn test_part_one() {
        let expected: Num = 55312;
        assert_eq!(expected, step_n(INPUT, 25))
    }
}
