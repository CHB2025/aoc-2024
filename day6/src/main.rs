use std::collections::HashSet;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Part one: {}", part_one(INPUT));
    println!("Part two: {}", part_two(INPUT));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn offset(self, mut pos: (usize, usize)) -> (usize, usize) {
        match self {
            Dir::Up => pos.1 -= 1,
            Dir::Down => pos.1 += 1,
            Dir::Left => pos.0 -= 1,
            Dir::Right => pos.0 += 1,
        };
        pos
    }
    fn next(self) -> Self {
        match self {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
        }
    }
}

fn part_one(input: &str) -> usize {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();
    println!("{width} x {height}");

    let mut map = HashSet::<(usize, usize)>::new();
    let mut visited = HashSet::<(usize, usize)>::new();
    let mut pos = (0, 0);
    let mut dir = Dir::Up;
    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            match c {
                '#' => _ = map.insert((x, y)),
                '^' => {
                    pos = (x, y);
                    dir = Dir::Up;
                }
                '>' => {
                    pos = (x, y);
                    dir = Dir::Right;
                }
                '<' => {
                    pos = (x, y);
                    dir = Dir::Left;
                }
                'v' => {
                    pos = (x, y);
                    dir = Dir::Down;
                }
                _ => (),
            }
        }
    }
    println!("{} barriers", map.len());
    visited.insert(pos);

    while (1..width - 1).contains(&pos.0) && (1..height - 1).contains(&pos.1) {
        if map.contains(&dir.offset(pos)) {
            dir = dir.next();
        }
        pos = dir.offset(pos);
        visited.insert(pos);
    }
    visited.len()
}

fn part_two(input: &str) -> usize {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();

    let mut map = HashSet::<(usize, usize)>::new();
    let mut visited_nod = HashSet::new();
    let mut visited = HashSet::<(usize, usize, Dir)>::new();
    let mut pos = (0, 0);
    let mut dir = Dir::Up;
    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            match c {
                '#' => _ = map.insert((x, y)),
                '^' => {
                    pos = (x, y);
                    dir = Dir::Up;
                }
                '>' => {
                    pos = (x, y);
                    dir = Dir::Right;
                }
                '<' => {
                    pos = (x, y);
                    dir = Dir::Left;
                }
                'v' => {
                    pos = (x, y);
                    dir = Dir::Down;
                }
                _ => (),
            }
        }
    }
    visited.insert((pos.0, pos.1, dir));
    visited_nod.insert(pos);

    let mut targets = HashSet::new();

    while (1..width - 1).contains(&pos.0) && (1..height - 1).contains(&pos.1) {
        // Check adding obstacle (only where guard has not been yet, since obstacle is placed before guard moves)
        if !visited_nod.contains(&dir.offset(pos)) && map.insert(dir.offset(pos)) {
            if check_path((width, height), &map, visited.clone(), pos, dir.next()) {
                targets.insert(dir.offset(pos));
            }
            map.remove(&dir.offset(pos));
        }

        if map.contains(&dir.offset(pos)) {
            dir = dir.next();
        } else {
            pos = dir.offset(pos);
        }
        visited.insert((pos.0, pos.1, dir));
        visited_nod.insert(pos);
    }
    targets.len()
}

fn check_path(
    (width, height): (usize, usize),
    map: &HashSet<(usize, usize)>,
    mut visited: HashSet<(usize, usize, Dir)>,
    start: (usize, usize),
    start_dir: Dir,
) -> bool {
    let mut pos = start;
    let mut dir = start_dir;
    visited.insert((pos.0, pos.1, dir));
    // println!("Checking path starting at {pos:?} going {dir:?}");
    while (1..width - 1).contains(&pos.0) && (1..height - 1).contains(&pos.1) {
        if map.contains(&dir.offset(pos)) {
            dir = dir.next();
        } else {
            pos = dir.offset(pos);
        }
        if !visited.insert((pos.0, pos.1, dir)) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_part_one() {
        let expected: usize = 41;
        assert_eq!(expected, part_one(INPUT))
    }

    #[test]
    fn test_part_two() {
        let expected: usize = 6;
        assert_eq!(expected, part_two(INPUT))
    }
}
