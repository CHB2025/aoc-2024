use std::collections::HashSet;

const INPUT: &str = include_str!("../input.txt");

type Num = usize;

fn main() {
    println!("Part one: {}", part_one(INPUT));
    println!("Part two: {}", part_two(INPUT));
}

fn score(trailhead: (Num, Num), map: &Vec<Vec<Num>>) -> Num {
    let mut paths = vec![vec![trailhead]];
    // look for h next to path.last(). If more than one, clone and add to end of paths
    for h in 1..10 {
        let mut visited_set = HashSet::new();
        paths = paths
            .into_iter()
            .flat_map(|v| {
                let &(x, y) = v.last().expect("Known to have min one element");
                let mut new_vec = Vec::new();
                // Up
                if y.checked_sub(1).map(|y| map[y][x]) == Some(h) {
                    let pt = (x, y - 1);
                    if visited_set.insert(pt) {
                        let mut p = v.clone();
                        p.push(pt);
                        new_vec.push(p);
                    }
                }
                // Down
                if map.get(y + 1).map(|row| row[x]) == Some(h) {
                    let pt = (x, y + 1);
                    if visited_set.insert(pt) {
                        let mut p = v.clone();
                        p.push(pt);
                        new_vec.push(p);
                    }
                }
                // Left
                if x.checked_sub(1).map(|x| map[y][x]) == Some(h) {
                    let pt = (x - 1, y);
                    if visited_set.insert(pt) {
                        let mut p = v.clone();
                        p.push(pt);
                        new_vec.push(p);
                    }
                }
                // Right
                if map[y].get(x + 1) == Some(&h) {
                    let pt = (x + 1, y);
                    if visited_set.insert(pt) {
                        let mut p = v.clone();
                        p.push(pt);
                        new_vec.push(p);
                    }
                }

                new_vec
            })
            .collect()
    }
    paths.len()
}

fn part_one(input: &str) -> Num {
    let topography = input
        .lines()
        .map(|l| l.bytes().map(|n| (n - b'0') as Num).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let trailheads = topography.iter().enumerate().flat_map(|(y, v)| {
        v.iter()
            .enumerate()
            .filter(|(_, h)| **h == 0)
            .map(move |(x, _)| (x, y))
    });

    // Lose some efficiency for overlapping paths
    // Can potentially improve using a map (x, y) -> known score
    trailheads
        .into_iter()
        .map(|th| score(th, &topography))
        .sum()
}

// Same as score, just without visited set
fn rating(trailhead: (Num, Num), map: &Vec<Vec<Num>>) -> Num {
    let mut paths = vec![vec![trailhead]];
    for h in 1..10 {
        paths = paths
            .into_iter()
            .flat_map(|v| {
                let &(x, y) = v.last().expect("Known to have min one element");
                let mut new_vec = Vec::new();
                // Up
                if y.checked_sub(1).map(|y| map[y][x]) == Some(h) {
                    let pt = (x, y - 1);
                    let mut p = v.clone();
                    p.push(pt);
                    new_vec.push(p);
                }
                // Down
                if map.get(y + 1).map(|row| row[x]) == Some(h) {
                    let pt = (x, y + 1);
                    let mut p = v.clone();
                    p.push(pt);
                    new_vec.push(p);
                }
                // Left
                if x.checked_sub(1).map(|x| map[y][x]) == Some(h) {
                    let pt = (x - 1, y);
                    let mut p = v.clone();
                    p.push(pt);
                    new_vec.push(p);
                }
                // Right
                if map[y].get(x + 1) == Some(&h) {
                    let pt = (x + 1, y);
                    let mut p = v.clone();
                    p.push(pt);
                    new_vec.push(p);
                }

                new_vec
            })
            .collect()
    }
    paths.len()
}

fn part_two(input: &str) -> Num {
    let topography = input
        .lines()
        .map(|l| l.bytes().map(|n| (n - b'0') as Num).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let trailheads = topography.iter().enumerate().flat_map(|(y, v)| {
        v.iter()
            .enumerate()
            .filter(|(_, h)| **h == 0)
            .map(move |(x, _)| (x, y))
    });

    // Lose some efficiency for overlapping paths
    // Can potentially improve using a map (x, y) -> known score
    trailheads
        .into_iter()
        .map(|th| rating(th, &topography))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test_part_one() {
        let expected: Num = 36;
        assert_eq!(expected, part_one(INPUT))
    }

    #[test]
    fn test_part_two() {
        let expected: Num = 81;
        assert_eq!(expected, part_two(INPUT))
    }
}
