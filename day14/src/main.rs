const INPUT: &str = include_str!("../input.txt");

const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;

#[derive(Debug, Clone)]
struct Robot {
    pos: (i32, i32),
    vel: (i32, i32),
}

impl Robot {
    fn tick(&mut self, width: i32, height: i32) {
        self.pos.0 += self.vel.0;
        if self.pos.0 < 0 {
            self.pos.0 += width;
        }
        if self.pos.0 >= width {
            self.pos.0 -= width;
        }

        self.pos.1 += self.vel.1;
        if self.pos.1 < 0 {
            self.pos.1 += height;
        }
        if self.pos.1 >= height {
            self.pos.1 -= height;
        }
    }
}

fn main() {
    println!("Part one: {}", part_one(INPUT, WIDTH, HEIGHT));
    println!("Part two: {}", part_two(INPUT, WIDTH, HEIGHT));
}

fn part_one(input: &str, width: i32, height: i32) -> usize {
    let mut robots = input
        .lines()
        .map(|rd| {
            let (pd, vd) = rd.split_once(' ').unwrap();
            let (px, py) = pd[2..].split_once(',').unwrap();
            let pos = (px.parse::<i32>().unwrap(), py.parse::<i32>().unwrap());
            let (vx, vy) = vd[2..].split_once(',').unwrap();
            let vel = (vx.parse::<i32>().unwrap(), vy.parse::<i32>().unwrap());
            Robot { pos, vel }
        })
        .collect::<Vec<_>>();
    for _ in 0..100 {
        for r in robots.iter_mut() {
            r.tick(width, height);
        }
    }

    let quadrant_counts = quadrants(&robots, width, height);
    println!("{:?}", quadrant_counts);
    quadrant_counts
        .into_iter()
        .reduce(|factor, next| factor * next)
        .expect("quadrant_counts has 4 elements")
}

fn print_robots(rbts: &[Robot], width: i32, height: i32) {
    for y in 0..height {
        for x in 0..width {
            let c = rbts.iter().filter(|r| r.pos == (x, y)).count();
            if c != 0 {
                print!("{c}");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

fn quadrants(rbts: &[Robot], width: i32, height: i32) -> [usize; 4] {
    rbts.iter()
        .filter_map(|r| {
            let x = if r.pos.0 > width / 2 { 1 } else { 0 };
            if width % 2 == 1 && r.pos.0 == width / 2 {
                return None;
            }
            let y = if r.pos.1 > height / 2 { 2 } else { 0 };
            if height % 2 == 1 && r.pos.1 == height / 2 {
                // might be wrong, but only odd sized grids here
                return None;
            }
            Some(x + y)
        })
        .fold([0usize; 4], |mut ql, q| {
            ql[q] += 1;
            ql
        })
}

fn part_two(input: &str, width: i32, height: i32) -> usize {
    let mut robots = input
        .lines()
        .map(|rd| {
            let (pd, vd) = rd.split_once(' ').unwrap();
            let (px, py) = pd[2..].split_once(',').unwrap();
            let pos = (px.parse::<i32>().unwrap(), py.parse::<i32>().unwrap());
            let (vx, vy) = vd[2..].split_once(',').unwrap();
            let vel = (vx.parse::<i32>().unwrap(), vy.parse::<i32>().unwrap());
            Robot { pos, vel }
        })
        .collect::<Vec<_>>();
    let mut min_danger = usize::MAX;
    let mut min_danger_robots = Vec::new();
    let mut min_danger_time = 0;
    for s in 1..10_000 {
        for r in robots.iter_mut() {
            r.tick(width, height);
        }
        let qc = quadrants(&robots, width, height);
        let danger = qc
            .into_iter()
            .reduce(|factor, next| factor * next)
            .expect("quadrant_counts has 4 elements");
        if danger < min_danger {
            min_danger = danger;
            min_danger_robots = robots.clone();
            min_danger_time = s;
        }
    }

    print_robots(&min_danger_robots, width, height);
    println!("");
    min_danger_time
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    fn test_part_one() {
        let expected: usize = 12;
        assert_eq!(expected, part_one(INPUT, 11, 7))
    }
}
