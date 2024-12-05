const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Part one: {}", part_one(INPUT));
    println!("Part two: {}", part_two(INPUT));
}

fn diagonals(input: &str) -> impl Iterator<Item = String> + use<'_> {
    let width = input.lines().next().unwrap().len();
    let height = input.len() / width;
    let diag = (0..width).map(move |s| {
        input
            .bytes()
            .skip(s)
            .step_by(width + 2)
            .take(width - s)
            .collect::<Vec<_>>()
    });
    let diag_bottom = (1..height).map(move |s| {
        input
            .bytes()
            .skip((width + 1) * s)
            .step_by(width + 2)
            .take(width - s)
            .collect::<Vec<_>>()
    });
    let anti_diag = (0..width).map(move |s| {
        input
            .bytes()
            .skip(s)
            .step_by(width)
            .take(s + 1)
            .collect::<Vec<_>>()
    });
    let anti_diag_bottom = (2..height + 1).map(move |s| {
        input
            .bytes()
            .skip((width + 1) * s - 2)
            .step_by(width)
            .take((width + 1) - s)
            .collect::<Vec<_>>()
    });
    diag.chain(diag_bottom)
        .chain(anti_diag)
        .chain(anti_diag_bottom)
        .map(|v| String::from_utf8(v).unwrap())
}

fn part_one(input: &str) -> usize {
    // split into lines, then columns, then diagonals
    // search each one for "XMAS"
    let width = input.lines().next().unwrap().len();
    let lines = input.lines().map(str::as_bytes).map(|s| s.to_vec());
    let columns = (0..width).map(|s| input.bytes().skip(s).step_by(width + 1).collect::<Vec<_>>());
    let diags = diagonals(input);
    lines
        .chain(columns)
        .map(|l| String::from_utf8(l).unwrap())
        .chain(diags)
        .map(|l| {
            let c = l.matches("XMAS").count() + l.matches("SAMX").count();
            c
        })
        .sum()
}

fn diagonals_with_indices(input: &str) -> impl Iterator<Item = (String, Vec<usize>)> + use<'_> {
    let width = input.lines().next().unwrap().len();
    let height = input.len() / width;
    let diag = (0..width).map(move |s| {
        input
            .bytes()
            .enumerate()
            .skip(s)
            .step_by(width + 2)
            .take(width - s)
            .fold((String::new(), Vec::new()), |(mut s, mut v), (i, char)| {
                s.push(char.into());
                v.push(i);
                (s, v)
            })
    });
    let diag_bottom = (1..height).map(move |s| {
        input
            .bytes()
            .enumerate()
            .skip((width + 1) * s)
            .step_by(width + 2)
            .take(width - s)
            .fold((String::new(), Vec::new()), |(mut s, mut v), (i, char)| {
                s.push(char.into());
                v.push(i);
                (s, v)
            })
    });
    let anti_diag = (0..width).map(move |s| {
        input
            .bytes()
            .enumerate()
            .skip(s)
            .step_by(width)
            .take(s + 1)
            .fold((String::new(), Vec::new()), |(mut s, mut v), (i, char)| {
                s.push(char.into());
                v.push(i);
                (s, v)
            })
    });
    let anti_diag_bottom = (2..height + 1).map(move |s| {
        input
            .bytes()
            .enumerate()
            .skip((width + 1) * s - 2)
            .step_by(width)
            .take((width + 1) - s)
            .fold((String::new(), Vec::new()), |(mut s, mut v), (i, char)| {
                s.push(char.into());
                v.push(i);
                (s, v)
            })
    });
    diag.chain(diag_bottom)
        .chain(anti_diag)
        .chain(anti_diag_bottom)
}

fn part_two(input: &str) -> usize {
    // look up "MAS" in diagonals, take index of 'A'
    // find duplicates
    let mut a_indices = Vec::new();
    diagonals_with_indices(input)
        .map(|(s, v)| {
            s.match_indices("MAS")
                .chain(s.match_indices("SAM"))
                .map(|(i, _)| v[i + 1])
                .collect::<Vec<_>>()
        })
        .for_each(|v| a_indices.extend_from_slice(&v));
    a_indices
        .iter()
        .enumerate()
        .filter(|(i, oi)| a_indices[i + 1..].contains(oi))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_part_one() {
        let expected: usize = 18;
        assert_eq!(expected, part_one(INPUT))
    }

    #[test]
    fn test_part_two() {
        let expected: usize = 9;
        assert_eq!(expected, part_two(INPUT))
    }
}
