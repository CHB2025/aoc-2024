const INPUT: &str = include_str!("../input.txt");

type Num = usize;

fn main() {
    println!("Part one: {}", part_one(INPUT));
    println!("Part two: {}", part_two(INPUT));
}

fn part_one(input: &str) -> Num {
    todo!()
}

fn part_two(input: &str) -> Num {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "";

    #[test]
    fn test_part_one() {
        let expected: Num = { todo!() };
        assert_eq!(expected, part_one(INPUT))
    }

    #[test]
    fn test_part_two() {
        let expected: Num = { todo!() };
        assert_eq!(expected, part_two(INPUT))
    }
}
