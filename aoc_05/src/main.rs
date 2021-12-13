use bresenham::Bresenham;

fn read_file(name: &str) -> Vec<String> {
    std::fs::read_to_string(name)
        .expect("file not found!")
        .lines()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn is_straight(x0: isize, y0: isize, x1: isize, y1: isize) -> bool {
    (x0 == x1) || (y0 == y1)
}

fn part_zero(file: &str, show_diag: bool) -> i32 {
    // change size depending on input
    let max: usize = if file == "example.txt" { 10 } else { 1000 };
    let input = read_file(file);
    //parses input into list of 4 numbers (or 2 coods)
    let lines: Vec<Vec<isize>> = input
        .into_iter()
        .map(|w| {
            w.replace(" -> ", ",")
                .split(',')
                .map(|v| v.parse().unwrap())
                .collect()
        })
        .collect();

    // screen to plot on
    let mut screen: Vec<Vec<isize>> = vec![vec![0; max]; max];

    //plot each point
    for l in lines {
        if is_straight(l[0], l[1], l[2], l[3]) || show_diag {
            for (x, y) in Bresenham::new((l[0], l[1]), (l[2], l[3])) {
                screen[y as usize][x as usize] += 1;
            }
            // this crate forgets to mark last pixel
            screen[l[3] as usize][l[2] as usize] += 1;
        }
    }

    screen.iter().flatten().filter(|x| **x > 1).count() as i32
}

fn part_one(file: &str) -> i32 {
    part_zero(file, false)
}

fn part_two(file: &str) -> i32 {
    part_zero(file, true)
}

fn main() {
    assert_eq!(5, part_one("example.txt"));
    println!("{}", part_one("input.txt"));
    //     println!("{:?}", plot_line(9,7,7,9,));

    assert_eq!(12, part_two("example.txt"));
    println!("{}", part_two("input.txt"));
}
