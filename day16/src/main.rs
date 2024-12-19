use std::collections::{BinaryHeap, HashMap, HashSet};

const INPUT: &str = include_str!("../input.txt");

type Num = usize;

fn main() {
    println!("Part one: {}", part_one(INPUT));
    println!("Part two: {}", part_two(INPUT));
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Path {
    steps: Vec<((Num, Num), Dir)>,
    facing: Dir,
    location: (Num, Num),
    score: Num,
}

impl Path {
    // Possible moves filtered by open spaces
    // Can loop, but that should be broken by heap
    fn options(&self, walls: &HashSet<(Num, Num)>) -> impl IntoIterator<Item = Path> {
        let forward = if !walls.contains(&self.facing.offset(self.location)) {
            let mut p = self.clone();
            p.location = p.facing.offset(p.location);
            p.score += 1;
            p.steps.push((p.location, p.facing));
            Some(p)
        } else {
            None
        };
        let left = if !walls.contains(&self.facing.left().offset(self.location)) {
            let mut p = self.clone();
            p.facing = p.facing.left();
            p.location = p.facing.offset(p.location);
            p.score += 1001;
            p.steps.push((p.location, p.facing));
            Some(p)
        } else {
            None
        };
        let right = if !walls.contains(&self.facing.right().offset(self.location)) {
            let mut p = self.clone();
            p.facing = p.facing.right();
            p.location = p.facing.offset(p.location);
            p.score += 1001;
            p.steps.push((p.location, p.facing));
            Some(p)
        } else {
            None
        };

        forward.into_iter().chain(left).chain(right)
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.score.partial_cmp(&other.score).map(|o| o.reverse())
    }
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score).reverse()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    Nor,
    East,
    Sou,
    West,
}

impl Dir {
    // Walls surround board, so not worried about overflow here
    fn offset(self, (x, y): (Num, Num)) -> (Num, Num) {
        match self {
            Dir::Nor => (x, y - 1),
            Dir::East => (x + 1, y),
            Dir::Sou => (x, y + 1),
            Dir::West => (x - 1, y),
        }
    }
    fn left(self) -> Self {
        match self {
            Dir::Nor => Dir::West,
            Dir::East => Dir::Nor,
            Dir::Sou => Dir::East,
            Dir::West => Dir::Sou,
        }
    }

    fn right(self) -> Self {
        match self {
            Dir::Nor => Dir::East,
            Dir::East => Dir::Sou,
            Dir::Sou => Dir::West,
            Dir::West => Dir::Nor,
        }
    }
}

fn part_one(input: &str) -> Num {
    let (map, start, end) = {
        let mut map = HashSet::new();
        let mut start = (0, 0);
        let mut end = (0, 0);
        for (y, l) in input.lines().enumerate() {
            for (x, c) in l.chars().enumerate() {
                match c {
                    '#' => _ = map.insert((x, y)),
                    'S' => start = (x, y),
                    'E' => end = (x, y),
                    _ => (),
                }
            }
        }

        (map, start, end)
    };
    assert!(start != (0, 0));
    assert!(end != (0, 0));

    let mut heap = BinaryHeap::<Path>::new();
    heap.push(Path {
        steps: vec![(start, Dir::East)],
        facing: Dir::East,
        location: start,
        score: 0,
    });
    let mut visited = HashSet::<(Num, Num, Dir)>::new();
    while let Some(p) = heap.pop() {
        if p.location == end {
            return p.score;
        }
        if !visited.insert((p.location.0, p.location.1, p.facing)) {
            continue;
        }
        heap.extend(p.options(&map));
    }
    unreachable!("Shouldn't reach this");
}

// Need to do cached dfs instead of bfs
fn part_two(input: &str) -> Num {
    let (map, start, end) = {
        let mut map = HashSet::new();
        let mut start = (0, 0);
        let mut end = (0, 0);
        for (y, l) in input.lines().enumerate() {
            for (x, c) in l.chars().enumerate() {
                match c {
                    '#' => _ = map.insert((x, y)),
                    'S' => start = (x, y),
                    'E' => end = (x, y),
                    _ => (),
                }
            }
        }

        (map, start, end)
    };
    assert!(start != (0, 0));
    assert!(end != (0, 0));

    let mut heap = BinaryHeap::<Path>::new();
    heap.push(Path {
        steps: vec![(start, Dir::East)],
        facing: Dir::East,
        location: start,
        score: 0,
    });

    let mut visited = HashMap::<((Num, Num), Dir), (Num, HashSet<((Num, Num), Dir)>)>::new();
    let mut min_score = None;
    loop {
        let p = heap.pop().unwrap();
        if let Some((dist, set)) = visited.get_mut(&(p.location, p.facing)) {
            if *dist == p.score {
                set.extend(p.steps);
            }
            continue;
        }
        if min_score.is_some_and(|s| s < p.score) {
            break;
        }
        visited.insert(
            (p.location, p.facing),
            (p.score, p.steps.iter().cloned().collect()),
        );
        if p.location == end {
            min_score = Some(p.score);
            continue;
        }
        heap.extend(p.options(&map));
    }
    let mut tiles: HashSet<((Num, Num), Dir)> = visited
        .get(&(end, Dir::Nor))
        .into_iter()
        .chain(visited.get(&(end, Dir::East)))
        .chain(visited.get(&(end, Dir::Sou)))
        .chain(visited.get(&(end, Dir::West)))
        .fold(HashSet::new(), |mut acc, (_, next)| {
            acc.extend(next.iter());
            acc
        });
    let mut prev_len = 0;
    // Necessary to get paths more than one level of indirection deep
    while prev_len < tiles.len() {
        prev_len = tiles.len();
        tiles = tiles
            .into_iter()
            .flat_map(|pt| visited.get(&pt).unwrap().1.iter().cloned())
            .collect::<HashSet<_>>();
    }
    tiles
        .into_iter()
        .map(|(pt, _)| pt)
        .collect::<HashSet<_>>()
        .len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

    const INPUT2: &str = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

    #[test]
    fn test_part_one() {
        let expected: Num = 7036;
        assert_eq!(expected, part_one(INPUT));

        let expected: Num = 11048;
        assert_eq!(expected, part_one(INPUT2));
    }

    #[test]
    fn test_part_two() {
        let expected: Num = 45;
        assert_eq!(expected, part_two(INPUT));

        let expected: Num = 64;
        assert_eq!(expected, part_two(INPUT2));
    }
}
