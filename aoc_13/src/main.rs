use std::collections::HashSet;

fn read_file(name: &str) -> Vec<String> {
    std::fs::read_to_string(name)
        .expect("file not found!")
        .lines()
        .map(|x| x.parse().unwrap())
        .collect()
}

#[derive(PartialEq, Debug)]
enum Dir {
    Horizontal,
    Vertical,
}

// x is vertical pos
// y is horizontal pos
// (x, y)
fn transfold_dots(dots: HashSet<(usize, usize)>, fold: &(Dir, usize)) -> HashSet<(usize, usize)> {
    let is_h = fold.0 == Dir::Horizontal;

    let n = dots
        .into_iter()
        .map(|(x, y)| {
            if       is_h && y > fold.1 { (x, 2 * fold.1 - y) } 
            else if !is_h && x > fold.1 { (2 * fold.1 - x, y) } 
            else                        { (x, y) }
        })
        .collect::<HashSet<(usize, usize)>>();

    n
}

fn print_dots(dots: &HashSet<(usize, usize)>, x: usize, y: usize) {
    let mut disp = vec![vec![false; x]; y];
    let mut s = String::new();
    for i in 0..y {
        s += &(0..x)
            .map(|j| if dots.contains(&(j, i)) { '#' } else { '.' })
            .collect::<String>();
        s += &"\n";
    }
    println!("{}", s);
}

fn sol(name: &str, is_pt1: bool) -> u32 {
    let input = read_file(name);
    let p1 = input.split(|l| l.is_empty()).nth(0).unwrap(); // dots
    let p2 = input.split(|l| l.is_empty()).nth(1).unwrap(); // folds
    let mut dots: HashSet<(usize, usize)> = p1 // parse into set of pairs
        .iter()
        .map(|l| {
            let d: Vec<usize> = l.split(',').map(|w| w.parse().unwrap()).collect();
            (d[0], d[1])
        })
        .collect();

    let folds: Vec<(Dir, usize)> = p2 // parse into vec of direction and line
        .iter()
        .map(|l: &String| {
            let instruc: Vec<&str> = l.split_whitespace().nth(2).unwrap().split('=').collect();
            let f_dir = match instruc[0] {
                "x" => Some(Dir::Vertical),
                "y" => Some(Dir::Horizontal),
                _ => None,
            };
            (f_dir.unwrap(), instruc[1].parse().unwrap())
        })
        .collect();

    if is_pt1 {
        // fold once
        dots = transfold_dots(dots, &folds[0]);
    } else {
        // fold all
        for fold in folds {
            dots = transfold_dots(dots, &fold);
        }
    }

    if !is_pt1 {
        // print with hardcoded size
        print_dots(&dots, 40, 6);
    }

    dots.len() as u32
}

fn main() {
    assert_eq!(17, sol("example.txt", true));

    println!("{}", sol("input.txt", true));

    sol("input.txt", false);
}
