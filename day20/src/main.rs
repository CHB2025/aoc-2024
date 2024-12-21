use std::{
    collections::{HashMap, HashSet},
    sync::{LazyLock, Mutex},
    time::Instant,
};

const INPUT: &str = include_str!("../input.txt");

type Num = usize;

fn main() {
    println!("Part one: {}", part_one(INPUT));
    let start = Instant::now();
    println!("Part two: {}", part_two(INPUT));
    println!("{}", start.elapsed().as_millis());
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    pub const fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
    pub fn checked_move(self, dir: Dir) -> Option<Self> {
        match dir {
            Dir::Nor => self.y.checked_sub(1).map(|y| Self::new(self.x, y)),
            Dir::East => self.x.checked_add(1).map(|x| Self::new(x, self.y)),
            Dir::Sou => self.y.checked_add(1).map(|y| Self::new(self.x, y)),
            Dir::West => self.x.checked_sub(1).map(|x| Self::new(x, self.y)),
        }
    }

    /// Manhattan distance between two points
    pub const fn dist(&self, other: &Self) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    /// Finds all points with a manhattan distance less than `dist`
    pub fn neighbors(self, dist: usize) -> impl Iterator<Item = Self> {
        if dist == 0 {
            panic!("neighbors only works with distances greater than 0");
        }
        Self::neighbor_dirs(dist)
            .into_iter()
            .filter(|dirs| dirs.len() != 1)
            .filter_map(move |dirs| {
                let mut pt = self;
                for d in dirs {
                    pt = pt.checked_move(d)?;
                }
                Some(pt)
            })
    }

    fn neighbor_dirs(dist: usize) -> Vec<Vec<Dir>> {
        static MAP: LazyLock<Mutex<HashMap<usize, Vec<Vec<Dir>>>>> =
            LazyLock::new(|| Mutex::new(HashMap::new()));
        MAP.lock()
            .unwrap()
            .entry(dist)
            .or_insert_with(|| {
                let mut directions = vec![
                    vec![Dir::Nor],
                    vec![Dir::West],
                    vec![Dir::Sou],
                    vec![Dir::East],
                ];
                for d in 1..dist {
                    for i in 0..directions.len() {
                        if directions[i].len() != d {
                            continue;
                        }

                        let last_dir = *directions[i].last().unwrap();
                        if directions[i].iter().all(|&d| d == last_dir) {
                            let mut nv = directions[i].clone();
                            nv.push(last_dir.left());
                            directions.push(nv.clone())
                        }
                        let mut nv = directions[i].clone();
                        nv.push(last_dir);
                        directions.push(nv.clone())
                    }
                }
                directions
            })
            .clone()
    }
}

const ALL_DIRS: [Dir; 4] = [Dir::Nor, Dir::East, Dir::Sou, Dir::West];
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Dir {
    Nor,
    East,
    Sou,
    West,
}

impl Dir {
    pub fn left(self) -> Self {
        match self {
            Dir::Nor => Dir::West,
            Dir::West => Dir::Sou,
            Dir::Sou => Dir::East,
            Dir::East => Dir::Nor,
        }
    }
    pub fn right(self) -> Self {
        match self {
            Dir::Nor => Dir::East,
            Dir::West => Dir::Nor,
            Dir::Sou => Dir::West,
            Dir::East => Dir::Sou,
        }
    }
}

fn part_one(input: &str) -> Num {
    let (mut start, mut end) = Default::default();
    let track = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| l.chars().enumerate().map(move |(x, c)| (x, y, c)))
        .filter_map(|(x, y, c)| match c {
            '.' => Some(Point::new(x, y)),
            'S' => {
                start = Point::new(x, y);
                Some(start)
            }
            'E' => {
                end = Point::new(x, y);
                Some(end)
            }
            _ => None,
        })
        .collect::<HashSet<_>>();
    let path = path(start, end, &track).unwrap();
    let mut cheats = HashMap::new();
    for (i, &s) in path.iter().enumerate() {
        // Should filter that e is after s
        s.neighbors(2)
            .filter_map(|e| path[i..].iter().position(|&p| p == e).map(|d| (e, d - 2)))
            .filter(|&(_, d)| d != 0)
            // .inspect(|(end, save)| println!("{s:?}-{end:?}: {save}"))
            .for_each(|(e, d)| _ = cheats.insert((s, e), d))
    }
    cheats.into_iter().filter(|(_, s)| *s >= 100).count()
}

/// Steps from a to b, including both a and b
fn path(a: Point, b: Point, track: &HashSet<Point>) -> Option<Vec<Point>> {
    if !track.contains(&a) || !track.contains(&b) {
        return None;
    }
    let dirs = ALL_DIRS
        .into_iter()
        .filter(|d| a.checked_move(*d).is_some_and(|pt| track.contains(&pt)));
    for d in dirs {
        let mut path = vec![a];
        let mut point = a;
        let mut dir = d;
        while let Some((p, d)) = next_point(point, dir, track) {
            point = p;
            dir = d;
            path.push(point);
            if point == b {
                return Some(path);
            }
        }
    }
    None
}

fn next_point(init: Point, dir: Dir, track: &HashSet<Point>) -> Option<(Point, Dir)> {
    if let Some(pt) = init.checked_move(dir).filter(|pt| track.contains(pt)) {
        return Some((pt, dir));
    }
    if let Some(pt) = init
        .checked_move(dir.left())
        .filter(|pt| track.contains(pt))
    {
        return Some((pt, dir.left()));
    }
    if let Some(pt) = init
        .checked_move(dir.right())
        .filter(|pt| track.contains(pt))
    {
        return Some((pt, dir.right()));
    }
    None
}

fn part_two(input: &str) -> Num {
    let (mut start, mut end) = Default::default();
    let track = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| l.chars().enumerate().map(move |(x, c)| (x, y, c)))
        .filter_map(|(x, y, c)| match c {
            '.' => Some(Point::new(x, y)),
            'S' => {
                start = Point::new(x, y);
                Some(start)
            }
            'E' => {
                end = Point::new(x, y);
                Some(end)
            }
            _ => None,
        })
        .collect::<HashSet<_>>();
    let path = path(start, end, &track).unwrap();

    let mut cheats = 0;
    for (i, &s) in path.iter().enumerate() {
        cheats += s
            .neighbors(20)
            .filter(|e| track.contains(e))
            .filter_map(|e| {
                path[i..]
                    .iter()
                    .position(|&p| p == e)
                    .map(|d| d - e.dist(&s))
            })
            .filter(|&d| d >= 100)
            .count();
    }
    cheats
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

    #[test]
    fn test_part_one() {
        let expected: Num = 0;
        assert_eq!(expected, part_one(INPUT))
    }

    #[test]
    fn test_part_two() {
        let expected: Num = 0;
        assert_eq!(expected, part_two(INPUT))
    }

    #[test]
    fn test_point_neighbors() {
        let pt = Point::new(5, 5);
        let neighbors = pt.neighbors(1).collect::<Vec<_>>();
        assert_eq!(neighbors.len(), 4);
        assert_eq!(
            neighbors,
            [
                Point { x: 5, y: 4 },
                Point { x: 4, y: 5 },
                Point { x: 5, y: 6 },
                Point { x: 6, y: 5 }
            ]
        );

        let neighbors2 = pt.neighbors(2).collect::<Vec<_>>();
        assert!(neighbors2.starts_with(&neighbors));
        assert_eq!(neighbors2.len(), 12);
        assert_eq!(
            neighbors2,
            [
                Point { x: 5, y: 4 },
                Point { x: 4, y: 5 },
                Point { x: 5, y: 6 },
                Point { x: 6, y: 5 },
                Point { x: 4, y: 4 },
                Point { x: 5, y: 3 },
                Point { x: 4, y: 6 },
                Point { x: 3, y: 5 },
                Point { x: 6, y: 6 },
                Point { x: 5, y: 7 },
                Point { x: 6, y: 4 },
                Point { x: 7, y: 5 }
            ]
        );

        let neighbors3 = pt.neighbors(3).collect::<Vec<_>>();
        assert_eq!(neighbors3.len(), 24);
        assert!(neighbors3.into_iter().all(|other| pt.dist(&other) <= 3));
    }
}
