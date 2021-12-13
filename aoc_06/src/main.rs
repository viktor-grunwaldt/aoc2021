fn read_file(name: &str) -> Vec<u8> {
    std::fs::read_to_string(name)
        .expect("file not found!")
        .split(',')
        .map(|x| x.parse::<i32>().unwrap() as u8)
        .collect()
}

fn part_one(name: &str, day: u32) -> u64 {
    let _fishes = read_file(name);
    let mut fishes_packed: Vec<u64> = vec![0; 9];
    // init fishes
    for fish in _fishes {
        fishes_packed[fish as usize] += 1;
    }
    // simulate growth
    for _i in 0..day {
        fishes_packed.rotate_left(1);
        fishes_packed[6] += fishes_packed[8];
    }

    fishes_packed.iter().sum()
}
fn main() {

    assert_eq!(26, part_one("example.txt", 18));
    assert_eq!(5934, part_one("example.txt", 80));
    assert_eq!(26984457539, part_one("example.txt", 256));

    println!("{}", part_one("input.txt", 80));
    println!("{}", part_one("input.txt", 256));
}
