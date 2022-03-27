fn read_file(name: &str) -> Vec<String> {
    std::fs::read_to_string(name)
        .expect("file not found!")
        .lines()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn part_one(name: &str) -> u32 {
    let input = read_file(name);


    0
}

fn main() {
    assert_eq!(0, part_one("example.txt"));
}

