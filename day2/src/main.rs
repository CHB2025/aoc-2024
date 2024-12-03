const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Part one: {}", part_one(INPUT));
    println!("Part two: {}", part_two(INPUT));
}

fn diffs(input: &str) -> impl Iterator<Item = Vec<i32>> + use<'_> {
    input
        .lines()
        .map(|l| {
            l.trim()
                .split(' ')
                .map(|i| i.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|report| {
            report
                .windows(2)
                .map(|sl| sl[1] - sl[0])
                .collect::<Vec<_>>()
        })
}

fn part_one(input: &str) -> usize {
    diffs(input)
        .filter(|diff| {
            diff.iter().all(|d| (1..=3).contains(d)) || diff.iter().all(|d| (-3..0).contains(d))
        })
        .count()
}

fn part_two(input: &str) -> usize {
    // Adding two diffs together gets diff if middle was dampened
    // Eg:
    // 1 5 2 3 gives
    // 4 -3 1
    // removing the 5 gives
    // 1 1
    // 4 + -3 = 1
    let mut safe = 0;
    for diff in diffs(input) {
        let count_pos = diff.iter().filter(|d| d.is_positive()).count();
        let range = if count_pos >= diff.len() - 1 {
            1..4
        } else {
            -3..0
        };
        let unsafe_ind = diff
            .iter()
            .map(|d| range.contains(d))
            .enumerate()
            .filter(|(_, s)| !s)
            .map(|(i, _)| i)
            .collect::<Vec<_>>();

        match unsafe_ind[..] {
            [] => {
                safe += 1;
            }
            [a] if a == 0 || a == diff.len() - 1 => {
                // ends can be cut
                safe += 1;
            }
            [a] if range.contains(&(diff[a] + diff[a + 1]))
                || range.contains(&(diff[a] + diff[a - 1])) =>
            {
                // check adding above or below
                safe += 1;
            }
            [a, b] if a + 1 == b && range.contains(&(diff[a] + diff[b])) => {
                // check adding together
                safe += 1;
            }
            _ => (), // still unsafe
        }
    }
    safe
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_part_one() {
        let expected: usize = 2;
        assert_eq!(expected, part_one(INPUT))
    }

    #[test]
    fn test_part_two() {
        let expected: usize = 4;
        assert_eq!(expected, part_two(INPUT))
    }
}
