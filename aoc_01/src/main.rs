use std::fs;

fn read_file(filename: &str) -> Vec<usize> {
    //reads file, splits into lines and converts into vec of ints
    return fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
        .split_whitespace()
        .map(|w| w.parse::<usize>().unwrap())
        .collect();
}

fn part_one(filename: &str) -> usize {
    let vec = read_file(filename);

    // windows() creates pairs of consecutive numbers
    // ex: [1,2,3].windows(2) = [[1,2],[2,3],[3,4]]
    // filter() removes an element if it's previous number is bigger
    // count() counts elements not removed by filter
    return vec.windows(2).filter(|w| w[0] < w[1]).count();
}

fn part_two(filename: &str) -> usize {
    let vec = read_file(filename);
    let sums: Vec<usize> = vec.windows(3).map(|w| w[0] + w[1] + w[2]).collect();

    return sums.windows(2).filter(|w| w[0] < w[1]).count();
}

fn main() {
    assert_eq!(7, part_one("example.txt"));
    assert_eq!(5, part_two("example.txt"));

    println!("{}", part_one("input.txt"));
    println!("{}", part_two("input.txt"));
}
