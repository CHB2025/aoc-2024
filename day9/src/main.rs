use std::iter;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Part one: {}", part_one(INPUT));
    println!("Part two: {}", part_two(INPUT));
}

type Num = usize;

fn part_one(input: &str) -> Num {
    let mut blocks: Vec<Option<Num>> = input
        .trim()
        .bytes()
        .enumerate()
        .flat_map(|(i, b)| {
            let val = if i % 2 == 0 {
                Some((i / 2) as Num)
            } else {
                None
            };
            iter::repeat(val).take((b - b'0') as Num)
        })
        .collect();

    let (mut start, mut end) = (0, blocks.len() - 1);
    while start <= end {
        if blocks[start].is_some() {
            start += 1;
            continue;
        }
        if blocks[end].is_none() {
            end -= 1;
            continue;
        }
        blocks.swap(start, end);
    }
    blocks
        .iter()
        .enumerate()
        .filter_map(|(i, b)| b.map(|id| id * i))
        .sum()
}

fn part_two(input: &str) -> Num {
    let mut blocks: Vec<Option<Num>> = input
        .trim()
        .bytes()
        .enumerate()
        .flat_map(|(i, b)| {
            let val = if i % 2 == 0 {
                Some((i / 2) as Num)
            } else {
                None
            };
            iter::repeat(val).take((b - b'0') as Num)
        })
        .collect();
    let mut prev = 0;
    // index, len
    let mut spaces: Vec<(Num, Num)> =
        blocks
            .iter()
            .enumerate()
            .filter(|(_, b)| b.is_none())
            .fold(Vec::new(), |mut v, (i, _)| {
                if i == prev + 1 {
                    v.last_mut().unwrap().1 += 1;
                } else {
                    v.push((i, 1));
                }
                prev = i;
                v
            });
    // index, len
    let mut prev = Num::MAX;
    let mut id = Num::MAX;
    let filesizes: Vec<(Num, Num)> = blocks
        .iter()
        .enumerate()
        .filter_map(|(i, b)| b.map(|b| (i, b)))
        .fold(Vec::new(), |mut v, (i, b)| {
            if i.checked_sub(prev) == Some(1) && b == id {
                v.last_mut().unwrap().1 += 1;
            } else {
                v.push((i, 1));
                id = b;
            }
            prev = i;
            v
        });

    for (fstart, flen) in filesizes.into_iter().rev() {
        let Some((sstart, slen)) = spaces.iter_mut().find(|(_, slen)| *slen >= flen) else {
            continue;
        };
        if fstart < *sstart {
            continue;
        }
        // swap fstart..fstart+flen with sstart..start+flen
        // update slen -= flen AND sstart += flen
        // slen = 0 not a problem bc it'll never be used
        let (space, file) = blocks.split_at_mut(fstart);
        file[..flen].swap_with_slice(&mut space[*sstart..*sstart + flen]);
        *slen -= flen;
        *sstart += flen;
    }

    blocks
        .iter()
        .enumerate()
        .filter_map(|(i, b)| b.map(|id| id * i))
        .sum()
}

// fn print(blocks: &Vec<Option<Num>>) {
//     let s: String = blocks
//         .iter()
//         .map(|b| b.map_or(".".to_owned(), |i| i.to_string()))
//         .collect();
//     println!("Map: {s}");
// }

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2333133121414131402";

    #[test]
    fn test_part_one() {
        let expected: Num = 1928;
        assert_eq!(expected, part_one(INPUT))
    }

    #[test]
    fn test_part_two() {
        let expected: Num = 2858;
        assert_eq!(expected, part_two(INPUT))
    }
}
