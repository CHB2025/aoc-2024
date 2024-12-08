use std::{
    collections::{HashMap, HashSet},
    iter,
};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Part one: {}", part_one(INPUT));
    println!("Part two: {}", part_two(INPUT));
}

fn part_one(input: &str) -> i32 {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();
    let map: HashMap<char, Vec<(i32, i32)>> = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(move |(x, c)| (x as i32, y as i32, c))
        })
        .filter(|(_, _, c)| *c != '.')
        .fold(HashMap::new(), |mut map, (x, y, c)| {
            map.entry(c).or_default().push((x, y));
            map
        });
    map.values()
        .flat_map(|pts| {
            pts.iter()
                .enumerate()
                .flat_map(|(i, p1)| pts[i + 1..].iter().flat_map(|p2| antinodes(*p1, *p2)))
        })
        .filter(|(x, y)| {
            (0..width).contains(&(*x as usize)) && (0..height).contains(&(*y as usize))
        })
        .fold(HashSet::new(), |mut set, pt| {
            set.insert(pt);
            set
        })
        .len() as i32
}

fn antinodes((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> [(i32, i32); 2] {
    let dx = x2 - x1;
    let dy = y2 - y1;
    [(x1 - dx, y1 - dy), (x2 + dx, y2 + dy)]
}

fn part_two(input: &str) -> i32 {
    let height = input.lines().count() as i32;
    let width = input.lines().next().unwrap().len() as i32;
    let map: HashMap<char, Vec<(i32, i32)>> = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(move |(x, c)| (x as i32, y as i32, c))
        })
        .filter(|(_, _, c)| *c != '.')
        .fold(HashMap::new(), |mut map, (x, y, c)| {
            map.entry(c).or_default().push((x, y));
            map
        });

    map.values()
        .flat_map(|pts| {
            pts.iter().enumerate().flat_map(|(i, p1)| {
                pts[i + 1..]
                    .iter()
                    .flat_map(|p2| antinodes2(*p1, *p2, width, height))
            })
        })
        .fold(HashSet::new(), |mut set, pt| {
            set.insert(pt);
            set
        })
        .len() as i32
}

fn antinodes2(
    (mut x1, mut y1): (i32, i32),
    (mut x2, mut y2): (i32, i32),
    width: i32,
    height: i32,
) -> impl Iterator<Item = (i32, i32)> {
    let dx = x2 - x1;
    let dy = y2 - y1;

    iter::once((x1, y1))
        .chain(iter::once((x2, y2)))
        .chain(
            iter::from_fn(move || {
                x1 -= dx;
                y1 -= dy;
                Some((x1, y1))
            })
            .take_while(move |(x, y)| (0..width).contains(x) && (0..height).contains(y)),
        )
        .chain(
            iter::from_fn(move || {
                x2 += dx;
                y2 += dy;
                Some((x2, y2))
            })
            .take_while(move |(x, y)| (0..width).contains(x) && (0..height).contains(y)),
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn test_antinodes() {
        let nodes = antinodes((0, 0), (1, 1));
        assert_eq!([(-1, -1), (2, 2)], nodes);

        let nodes = antinodes((0, 0), (1, 2));
        assert_eq!([(-1, -2), (2, 4)], nodes);
    }

    #[test]
    fn test_part_one() {
        let expected: i32 = 14;
        assert_eq!(expected, part_one(INPUT))
    }

    #[test]
    fn test_part_two() {
        let expected: i32 = 34;
        assert_eq!(expected, part_two(INPUT))
    }
}
