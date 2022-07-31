fn read_file(name: &str) -> Vec<String> {
    std::fs::read_to_string(name)
        .expect("file not found!")
        .lines()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn part_one(name: &str) -> i32 {
    let mut depth = 0;
    let mut dist = 0;
    for i in read_file(name) {
        let split: Vec<&str> = i.split_whitespace().collect();
        let val: i32 = split[1].parse().expect("couldn't parse value");
        match split[0] {
            "forward" => dist += val,
            "down" => depth += val,
            "up" => depth -= val,
            _ => (),
        }
    }
    depth * dist
}

fn part_two(name: &str) -> i32 {
    let mut aim = 0;
    let mut dist = 0;
    let mut depth = 0;
    for i in read_file(name) {
        let (dir, str_val) = i.split_once(' ').unwrap();
        let val: i32 = str_val.parse().expect("couldn't parse value");
        match dir {
            "down" => aim += val,
            "up" => aim -= val,
            "forward" => {
                dist += val;
                depth += aim * val;
            }
            _ => (),
        }
    }
    depth * dist
}

fn main() {
    assert_eq!(150, part_one("example.txt"));
    println!("{}", part_one("input.txt"));
    assert_eq!(900, part_two("example.txt"));
    println!("{}", part_two("input.txt"));
}
