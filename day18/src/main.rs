use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    fmt::Display,
};

const INPUT: &str = include_str!("../input.txt");

#[cfg(not(test))]
const SIZE: usize = 70;
#[cfg(test)]
const SIZE: usize = 6;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn neighbors(&self) -> impl Iterator<Item = Self> {
        self.x
            .checked_sub(1)
            .map(|x| Point { x, y: self.y })
            .into_iter()
            .chain(self.y.checked_sub(1).map(|y| Self { x: self.x, y }))
            .chain(self.x.checked_add(1).map(|x| Self { x, y: self.y }))
            .chain(self.y.checked_add(1).map(|y| Self { x: self.x, y }))
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

fn main() {
    println!("Part one: {}", part_one(INPUT, 1024));
    println!("Part two: {}", part_two(INPUT, 1024));
}

fn part_one(input: &str, bytes: usize) -> usize {
    let mut map = (0..=SIZE)
        .flat_map(|x| (0..=SIZE).map(move |y| Point { x, y }))
        .collect::<HashSet<_>>();

    for pt in input
        .lines()
        .take(bytes)
        .map(|l| l.split_once(',').unwrap())
        .map(|(x, y)| Point {
            x: x.parse::<usize>().unwrap(),
            y: y.parse::<usize>().unwrap(),
        })
    {
        map.remove(&pt);
    }

    find(Point { x: 0, y: 0 }, Point { x: SIZE, y: SIZE }, &map).unwrap()
}

fn find(start: Point, end: Point, map: &HashSet<Point>) -> Option<usize> {
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0usize, start)));
    let mut unvisited = map.clone();
    loop {
        let Reverse((dist, pt)) = heap.pop()?;
        if !unvisited.remove(&pt) {
            continue;
        }
        if pt == end {
            return Some(dist);
        }
        heap.extend(
            pt.neighbors()
                .filter_map(|pt| {
                    if unvisited.contains(&pt) {
                        Some((dist + 1, pt))
                    } else {
                        None
                    }
                })
                .map(Reverse),
        );
    }
}

// Could save path, only check if byte that fell is on the path
fn part_two(input: &str, bytes: usize) -> Point {
    let mut map = (0..=SIZE)
        .flat_map(|x| (0..=SIZE).map(move |y| Point { x, y }))
        .collect::<HashSet<_>>();

    let mut falling_bytes = input
        .lines()
        .map(|l| l.split_once(',').unwrap())
        .map(|(x, y)| Point {
            x: x.parse::<usize>().unwrap(),
            y: y.parse::<usize>().unwrap(),
        });
    for _ in 0..bytes {
        map.remove(&falling_bytes.next().unwrap());
    }
    let mut byte = falling_bytes.next().unwrap();
    map.remove(&byte);
    while find(Point { x: 0, y: 0 }, Point { x: SIZE, y: SIZE }, &map).is_some() {
        byte = falling_bytes.next().unwrap();
        map.remove(&byte);
    }
    byte
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

    #[test]
    fn test_part_one() {
        let expected: usize = 22;
        assert_eq!(expected, part_one(INPUT, 12))
    }

    #[test]
    fn test_part_two() {
        let expected = Point { x: 6, y: 1 };
        assert_eq!(expected, part_two(INPUT, 12))
    }
}
