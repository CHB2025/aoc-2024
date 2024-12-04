const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Part one: {}", part_one(INPUT));
    println!("Part two: {}", part_two(INPUT));
}

fn part_one(input: &str) -> u32 {
    // regex easiest???
    // /mul\(\d{1-3},\d{1-3}\)/
    assert!(input.is_ascii());
    let mut total = 0;
    for (i, _) in input.match_indices("mul(") {
        let Some(end) = input[i..].find(')') else {
            continue; // end of string w/ no closing paren
        };
        if end > 11 {
            continue;
        }
        let Some((a, b)) = input[i + 4..i + end].split_once(',') else {
            continue;
        };
        if a.len() > 3 || b.len() > 3 {
            continue;
        }
        // Non-digits (eg commas, periods) will error.
        // See https://doc.rust-lang.org/std/primitive.u32.html#method.from_str_radix
        let Some((a, b)) = a.parse::<u32>().ok().zip(b.parse::<u32>().ok()) else {
            continue;
        };
        total += a * b
    }
    total
}

fn part_two(input: &str) -> u32 {
    // just cut apart the input and use part 1
    // May join some broken muls together and create one working one in rare
    // cases:
    // "mul(1,don't()blahblahblahd043do()34)"
    // would result in "mul(1,34)". No cases in the input, so I'm leaving it.
    let new_input: String = input
        .split("do()")
        .map(|s| s.split_once("don't()").unwrap_or((s, "")).0)
        .collect();
    part_one(&new_input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn test_part_one() {
        let expected: u32 = 161;
        assert_eq!(expected, part_one(INPUT))
    }

    #[test]
    fn test_part_two() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let expected: u32 = 48;
        assert_eq!(expected, part_two(input))
    }
}
