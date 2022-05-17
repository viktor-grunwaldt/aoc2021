use itertools::Itertools;

fn read_file(name: &str) -> String {
    std::fs::read_to_string(name).expect("file not found!")
}
#[derive(Debug)]
struct Rect {
    x_start: i32,
    y_start: i32,
    x_end: i32,
    y_end: i32,
}

impl Rect {
    fn contains(&self, p: &(i32, i32)) -> bool {
        self.x_start <= p.0 && p.0 <= self.x_end 
        && self.y_start <= p.1 && p.1 <= self.y_end
    }
}

fn calc_trajectory(vel: (i32, i32), goal: &Rect) -> bool {
    let (mut x_vel, mut y_vel) = vel;
    let mut pos = (0, 0);
    loop {
        if goal.contains(&pos) {
            return true;
        } else if goal.x_end < pos.0 || pos.1 < goal.y_start {
            return false;
        }
        pos = (pos.0 + x_vel, pos.1 + y_vel);
        x_vel -= x_vel.signum();
        y_vel -= 1;
    }
}

fn part_one(name: &str) -> i32 {
    // target area: x=20..30, y=-10..-5
    let input = read_file(name);
    let coords: Vec<i32> = input
        .trim()
        .trim_start_matches("target area: x=")
        .split(", y=")
        .flat_map(|x| 
            x.split("..")
            .map(|xs| xs.parse().unwrap()))
        .collect();
    
    let target = Rect {
        x_start: coords[0],
        x_end:   coords[1],
        y_start: coords[2],
        y_end:   coords[3],
    };

    target.y_start * (target.y_start + 1) / 2
}

fn part_two(name: &str) -> u32 {
    // target area: x=20..30, y=-10..-5
    let input = read_file(name);
    let coords: Vec<i32> = input
        .trim()
        .trim_start_matches("target area: x=")
        .split(", y=")
        .flat_map(|x| 
            x.split("..")
            .map(|xs| xs.parse().unwrap()))
        .collect();
    
    let target = Rect {
        x_start: coords[0],
        x_end:   coords[1],
        y_start: coords[2],
        y_end:   coords[3],
    };
    let y = target.y_start.abs();

    let shoots = (0..=target.x_end)
        .cartesian_product(-y..y)
        .filter(|&p| calc_trajectory(p, &target))
        .count();

    shoots as u32
}
fn main() {
    println!("{}", part_one("input.txt"));
    println!("{}", part_two("input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;
    const TARGET: Rect = Rect {
        x_start: 20,
        x_end: 30,
        y_start: -10,
        y_end: -5,
    };

    #[test]
    fn test_bound_check() {
        assert!(TARGET.contains(&(28, -7)))
    }
    #[test]
    fn test_calc_trajectory() {
        assert!(calc_trajectory((7, 2), &TARGET));
        assert!(calc_trajectory((6, 3), &TARGET));
        assert!(calc_trajectory((9, 0), &TARGET));
        assert!(!calc_trajectory((17, -4), &TARGET));
    }

    #[test]
    fn test_outliers() {
        assert!(calc_trajectory((6, 0), &TARGET));
    }
    #[test]
    fn test_part_one() {
        assert_eq!(part_one("example.txt"), 45);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two("example.txt"), 112);
    }
}
