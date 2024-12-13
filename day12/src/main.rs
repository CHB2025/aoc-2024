use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../input.txt");

type Num = usize;

// Could do a lot of clean up on part two, but I'm not going to
fn main() {
    println!("Part one: {}", part_one(INPUT));
    println!("Part two: {}", part_two(INPUT));
}

type PlotMap = HashMap<(usize, usize), (char, Num)>;

#[derive(Debug)]
struct Region {
    plots: Vec<(Num, Num)>,
    perimeter: Num,
}
type RegionMap = HashMap<Num, Region>;

fn part_one(input: &str) -> Num {
    let mut plots = PlotMap::new();
    let mut regions = RegionMap::new();
    let mut next_region_id: Num = 0;

    for (x, y, c) in input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| l.chars().enumerate().map(move |(x, c)| (x, y, c)))
    {
        let mut perimeter = 4;
        let mut pid = Num::MAX;
        if let Some(&(_, p)) = y
            .checked_sub(1)
            .and_then(|y| plots.get(&(x, y)))
            .filter(|&&(k, _)| k == c)
        {
            perimeter -= 2; // One for this square, one for neighbor
            pid = p;
        }

        if let Some(&(_, p)) = x
            .checked_sub(1)
            .and_then(|x| plots.get(&(x, y)))
            .filter(|&&(k, _)| k == c)
        {
            perimeter -= 2; // One for this square, one for neighbor
            if pid != Num::MAX && pid != p {
                // merge two regions
                let other = regions.remove(&p).unwrap();
                for pt in &other.plots {
                    plots.get_mut(pt).unwrap().1 = pid;
                }
                let kept = regions.get_mut(&pid).unwrap();

                kept.plots.extend_from_slice(&other.plots);
                kept.perimeter += other.perimeter;
            } else {
                pid = p;
            }
        }
        if pid == Num::MAX {
            pid = next_region_id;
            next_region_id += 1;
        }
        plots.insert((x, y), (c, pid));
        let r = regions.entry(pid).or_insert(Region {
            plots: Vec::new(),
            perimeter: 0,
        });
        r.plots.push((x, y));
        r.perimeter += perimeter;
    }

    regions
        .into_iter()
        .map(|(_, r)| r.plots.len() * r.perimeter)
        .sum()
}

fn part_two(input: &str) -> Num {
    // The way I did part one doesn't seem particularly helpful
    let mut regions: HashMap<Num, Vec<(Num, Num)>> = HashMap::new();
    let mut plots = PlotMap::new();
    let mut next_region_id = 0;
    for (x, y, c) in input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| l.chars().enumerate().map(move |(x, c)| (x, y, c)))
    {
        let mut pid = Num::MAX;
        if let Some(&(_, p)) = y
            .checked_sub(1)
            .and_then(|y| plots.get(&(x, y)))
            .filter(|&&(k, _)| k == c)
        {
            pid = p;
        }

        if let Some(&(_, p)) = x
            .checked_sub(1)
            .and_then(|x| plots.get(&(x, y)))
            .filter(|&&(k, _)| k == c)
        {
            if pid != Num::MAX && pid != p {
                // merge two regions
                let other = regions.remove(&p).unwrap();
                for pt in &other {
                    plots.get_mut(pt).unwrap().1 = pid;
                }
                regions.get_mut(&pid).unwrap().extend_from_slice(&other);
            } else {
                pid = p;
            }
        }
        if pid == Num::MAX {
            pid = next_region_id;
            next_region_id += 1;
        }
        plots.insert((x, y), (c, pid));
        regions.entry(pid).or_insert(Vec::new()).push((x, y));
    }

    regions
        .into_iter()
        .map(|(i, v)| {
            let area = v.len();
            let start = *v.first().unwrap();
            let set = HashSet::from_iter(v.into_iter());
            let sc = collect_holes(start, &set) + count_sides(start, set);
            sc * area
        })
        .sum()
}

type Offset = Box<dyn Fn((Num, Num)) -> (Num, Num)>;
// plot above starting plot must not be in the region
fn count_sides(start: (Num, Num), plots: HashSet<(Num, Num)>) -> Num {
    // How do we handle loops
    if plots.len() == 1 || plots.len() == 2 {
        return 4;
    }
    let mut side_count = 0;
    // Start going right
    let mut left: Offset = Box::new(|(x, y)| (x, y.wrapping_sub(1)));
    let mut forward: Offset = Box::new(|(x, y)| (x + 1, y));
    let mut right: Offset = Box::new(|(x, y)| (x, y + 1));
    let mut back: Offset = Box::new(|(x, y)| (x.wrapping_sub(1), y));

    assert!(!plots.contains(&left(start)));
    assert!(!plots.contains(&back(start)));

    let mut pt = start;
    let start_forward = forward(start);
    loop {
        if plots.contains(&left(pt)) {
            // Turning left
            pt = left(pt);
            side_count += 1;
            // Rotate left
            (left, forward, right, back) = (back, left, forward, right);
        } else if plots.contains(&forward(pt)) {
            pt = forward(pt);
        } else {
            // Turning right
            side_count += 1;
            // Rotate right
            (left, forward, right, back) = (forward, right, back, left);
        }
        if pt == start && start_forward == forward(pt) {
            break side_count;
        }
    }
}

// plot above starting plot must not be in the region
fn perimeter_top(start: (Num, Num), plots: &HashSet<(Num, Num)>) -> (Num, HashSet<(Num, Num)>) {
    if plots.len() == 1 || plots.len() == 2 {
        return (4, plots.clone());
    }
    let mut side_count = 0;
    let mut tops = HashSet::new();
    // Start going right
    let mut dir: usize = 0;
    let mut left: Offset = Box::new(|(x, y)| (x, y.wrapping_sub(1)));
    let mut forward: Offset = Box::new(|(x, y)| (x + 1, y));
    let mut right: Offset = Box::new(|(x, y)| (x, y + 1));
    let mut back: Offset = Box::new(|(x, y)| (x.wrapping_sub(1), y));

    assert!(!plots.contains(&left(start)));

    let mut pt = start;
    tops.insert(pt);
    let start_forward = forward(start);
    loop {
        if plots.contains(&left(pt)) {
            // Turning left
            pt = left(pt);
            dir = dir.checked_sub(1).unwrap_or(3);
            if dir == 0 {
                tops.insert(pt);
            }
            side_count += 1;
            // Rotate left
            (left, forward, right, back) = (back, left, forward, right);
        } else if plots.contains(&forward(pt)) {
            pt = forward(pt);
            if !plots.contains(&left(pt)) && dir == 0 {
                tops.insert(pt);
            }
        } else {
            // Rotate right
            side_count += 1;
            dir = (dir + 1) % 4;
            if dir == 0 {
                tops.insert(pt);
            }
            (left, forward, right, back) = (forward, right, back, left);
        }
        if pt == start && start_forward == forward(pt) {
            break (side_count, tops);
        }
    }
}

// plot above starting plot must not be in the region
fn collect_holes(start: (Num, Num), plots: &HashSet<(Num, Num)>) -> Num {
    let mut used: HashSet<(Num, Num)> = perimeter_top(start, plots).1;
    let mut sides = 0;
    for &pt in plots {
        if used.contains(&pt) || plots.contains(&(pt.0, pt.1.wrapping_sub(1))) {
            continue;
        }
        // Not used so far (eg top of perimeter) and space above not in region: Hole
        let (sc, set) = perimeter_top(pt, plots);
        used.extend(set.into_iter());
        sides += sc;
    }

    sides
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    #[test]
    fn test_part_one() {
        let expected: Num = 1930;
        assert_eq!(expected, part_one(INPUT))
    }

    #[test]
    fn test_part_two() {
        let expected: Num = 1206;
        assert_eq!(expected, part_two(INPUT))
    }

    #[test]
    fn test_part_two_alt() {
        let input = "AAAA
BBCD
BBCC
EEEC";
        assert_eq!(80, part_two(input));
    }

    #[test]
    fn test_part_two_third() {
        let input = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";
        assert_eq!(236, part_two(input));
    }

    #[test]
    fn test_part_two_fourth() {
        let input = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";

        assert_eq!(368, part_two(input));
    }

    #[test]
    fn test_perimeter_tops() {
        let mut set = HashSet::from_iter([(1, 1), (2, 1), (3, 1)]);
        let orig_set = set.clone();
        let (sides, tops) = perimeter_top((1, 1), &set);
        assert_eq!(tops, set);
        assert!(sides == 4);

        set.insert((1, 2));
        set.insert((1, 3));
        set.insert((2, 3));
        set.insert((3, 3));
        set.insert((3, 2));

        let (sides, tops) = perimeter_top((1, 1), &set);
        assert_eq!(tops, orig_set);
        assert!(sides == 4);

        let bottom_set = HashSet::from_iter([(2, 3)].into_iter());
        let (sides, tops) = perimeter_top((2, 3), &bottom_set);
        assert_eq!(tops, bottom_set);
        assert!(sides == 4);
    }

    #[test]
    fn test_holes() {
        let mut set = HashSet::new();
        set.insert((1, 1));
        set.insert((2, 1));
        set.insert((3, 1));
        assert!(collect_holes((1, 1), &set) == 0);
        set.insert((1, 2));
        set.insert((1, 3));
        set.insert((2, 3));
        set.insert((3, 3));
        set.insert((3, 2));
        let t = collect_holes((1, 1), &set);
        assert!(t == 4);
        set.insert((1, 4));
        set.insert((1, 5));
        set.insert((2, 5));
        set.insert((3, 5));
        set.insert((3, 4));
        assert!(collect_holes((1, 1), &set) == 8);
    }
}
