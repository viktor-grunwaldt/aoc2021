fn read_file(name: &str) -> Vec<i32> {
    std::fs::read_to_string(name)
        .expect("file not found!")
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}

fn part_one(name: &str) -> i32 {
    let points = read_file(name);

    let min = points.iter().min().unwrap();
    let max = points.iter().max().unwrap();
    //let's see how fast rust is
    // so basically calculate all paths from min to max and pick the smallest one
    
    (*min..*max)
        .map(|w| points.iter().fold(0, |acc, x| acc + (x - w).abs()))
        .min()
        .unwrap()
}

fn part_two(name: &str) -> i32 {
    let points = read_file(name);
    fn f(x: i32) -> i32 {
        if 0 > x {
            panic!("invalid path");
        }
        x * (x + 1) / 2
    }

    let min = points.iter().min().unwrap();
    let max = points.iter().max().unwrap();
    //let's see how fast rust is
    // so basically calculate all paths from min to max and pick the smallest one
    
    (*min..*max)
        .map(|w| points.iter().fold(0, |acc, x| acc + f((x - w).abs())))
        .min()
        .unwrap()
}

fn main() {
    assert_eq!(37, part_one("example.txt"));
    println!("{}", part_one("input.txt"));
    assert_eq!(168, part_two("example.txt"));
    println!("{}", part_two("input.txt"));
}
