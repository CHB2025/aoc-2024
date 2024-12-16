use std::collections::{BTreeSet, HashMap, HashSet};

const INPUT: &str = include_str!("../input.txt");

type Num = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Obj {
    Wall,
    Box,
}

fn main() {
    println!("Part one: {}", part_one(INPUT));
    println!("Part two: {}", part_two(INPUT));
}

fn offset(dir: char, (x, y): (Num, Num)) -> (Num, Num) {
    match dir {
        '<' => (x - 1, y),
        '^' => (x, y - 1),
        '>' => (x + 1, y),
        'v' => (x, y + 1),
        _ => panic!("Unexpected dir: {}", dir),
    }
}

fn rev_offset(dir: char, (x, y): (Num, Num)) -> (Num, Num) {
    match dir {
        '<' => (x + 1, y),
        '^' => (x, y + 1),
        '>' => (x - 1, y),
        'v' => (x, y - 1),
        _ => panic!("Unexpected dir: {}", dir),
    }
}

fn part_one(input: &str) -> Num {
    let mut robot: (Num, Num) = (0, 0);
    let (text_map, movements) = input.split_once("\n\n").unwrap();
    let mut map: HashMap<(Num, Num), Obj> = text_map
        .lines()
        .enumerate()
        .flat_map(|(y, l)| l.chars().enumerate().map(move |(x, c)| (x, y, c)))
        .filter_map(|(x, y, c)| match c {
            '#' => Some(((x, y), Obj::Wall)),
            'O' => Some(((x, y), Obj::Box)),
            '@' => {
                robot = (x, y);
                None
            }
            _ => None,
        })
        .collect();
    assert!(robot != (0, 0));
    for m in movements.chars().filter(|c| *c != '\n') {
        let mut pt = offset(m, robot);
        while map.get(&pt) == Some(&Obj::Box) {
            pt = offset(m, pt);
        }
        let may_move = map.get(&pt).is_none();
        if !may_move {
            continue;
        }
        robot = offset(m, robot);
        while let Some(obj) = map.remove(&rev_offset(m, pt)) {
            map.insert(pt, obj);
            pt = rev_offset(m, pt);
        }
    }

    map.into_iter()
        .filter_map(|((x, y), o)| {
            if o == Obj::Box {
                Some(100 * y + x)
            } else {
                None
            }
        })
        .sum()
}

fn part_two(input: &str) -> Num {
    let mut robot: (Num, Num) = (0, 0);
    let (text_map, movements) = input.split_once("\n\n").unwrap();
    // Boxes and walls are indicated by Objs in their left space
    let mut map: HashMap<(Num, Num), Obj> = text_map
        .lines()
        .enumerate()
        .flat_map(|(y, l)| l.chars().enumerate().map(move |(x, c)| (x * 2, y, c)))
        .filter_map(|(x, y, c)| match c {
            '#' => Some(((x, y), Obj::Wall)),
            'O' => Some(((x, y), Obj::Box)),
            '@' => {
                robot = (x, y);
                None
            }
            _ => None,
        })
        .collect();
    'outer: for m in movements.chars().filter(|c| *c != '\n') {
        let mut pts_to_check = BTreeSet::new();
        pts_to_check.insert(offset(m, robot)); // pushing on this square
        pts_to_check.insert(offset('<', offset(m, robot))); // If this is a Box, it is in the adjacent square too
        let mut checked = HashSet::new();

        // Should this be first, last? Does it matter
        while let Some(pt) = pts_to_check.pop_first() {
            checked.insert(pt);
            match map.get(&pt) {
                None => (), // Nothing to worry about, keep checking
                Some(Obj::Box) => {
                    let new_pt = offset(m, pt);
                    if !checked.contains(&new_pt) {
                        pts_to_check.insert(new_pt);
                    }
                    if !checked.contains(&offset('>', new_pt)) {
                        // Right since this box extends to the right from the label
                        pts_to_check.insert(offset('>', new_pt));
                    }
                    if !checked.contains(&offset('<', new_pt)) {
                        // Left since a box/wall here extends above this point
                        pts_to_check.insert(offset('<', new_pt));
                    }
                }
                Some(Obj::Wall) => continue 'outer,
            }
        }

        robot = offset(m, robot);
        push(&mut map, robot, m);
        push(&mut map, offset('<', robot), m);
    }
    map.into_iter()
        .filter_map(|((x, y), o)| {
            if o == Obj::Box {
                Some(100 * y + x)
            } else {
                None
            }
        })
        .sum()
}

fn push(map: &mut HashMap<(Num, Num), Obj>, pt: (Num, Num), dir: char) {
    let Some(obj) = map.remove(&pt) else {
        return;
    };
    let dest = offset(dir, pt);
    push(map, offset('<', dest), dir);
    push(map, dest, dir);
    push(map, offset('>', dest), dir);
    map.insert(dest, obj);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    #[test]
    fn test_part_one() {
        let expected: Num = 10092;
        assert_eq!(expected, part_one(INPUT))
    }

    #[test]
    fn test_part_two() {
        let expected: Num = 9021;
        assert_eq!(expected, part_two(INPUT))
    }
}
