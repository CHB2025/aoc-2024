const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Part one: {}", part_one(INPUT));
    println!("Part two: {}", part_two(INPUT));
}

fn totals(running: u64, nums: &[u64]) -> Vec<u64> {
    let Some(next) = nums.first() else {
        return vec![running];
    };
    let mut v = totals(running + next, &nums[1..]);
    v.extend_from_slice(&totals(running * next, &nums[1..]));
    v
}

fn part_one(input: &str) -> u64 {
    input
        .lines()
        .map(|l| {
            let (exp, nums) = l.split_once(':').unwrap();
            let exp = exp.parse::<u64>().unwrap();
            let nums = nums
                .trim()
                .split(' ')
                .map(|n| n.parse::<u64>().unwrap())
                .collect::<Vec<_>>();
            (exp, nums)
        })
        .filter(|(exp, nums)| totals(nums[0], &nums[1..]).contains(exp))
        .map(|(e, _)| e)
        .sum()
}

fn totals2(running: u64, nums: &[u64], max: u64) -> Vec<u64> {
    if running > max {
        return Vec::new();
    }
    let Some(next) = nums.first() else {
        return vec![running];
    };
    let mut v = totals2(running + next, &nums[1..], max);
    v.extend_from_slice(&totals2(running * next, &nums[1..], max));
    v.extend_from_slice(&totals2(
        format!("{running}{next}").parse().unwrap(),
        &nums[1..],
        max,
    ));
    v
}

fn part_two(input: &str) -> u64 {
    input
        .lines()
        .map(|l| {
            let (exp, nums) = l.split_once(':').unwrap();
            let exp = exp.parse::<u64>().unwrap();
            let nums = nums
                .trim()
                .split(' ')
                .map(|n| n.parse::<u64>().unwrap())
                .collect::<Vec<_>>();
            (exp, nums)
        })
        .filter(|(exp, nums)| totals2(nums[0], &nums[1..], *exp).contains(exp))
        .map(|(e, _)| e)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_part_one() {
        let expected: u64 = 3749;
        assert_eq!(expected, part_one(INPUT))
    }

    #[test]
    fn test_part_two() {
        let expected: u64 = 11387;
        assert_eq!(expected, part_two(INPUT))
    }
}
