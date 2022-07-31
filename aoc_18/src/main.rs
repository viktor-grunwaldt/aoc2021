fn read_file(name: &str) -> Vec<String> {
    std::fs::read_to_string(name)
        .expect("file not found!")
        .lines()
        .map(|x| x.to_string())
        .collect()
}

fn parse_line(line: String) -> Vec<(u8, u8)> {
    // precalculate depth of number
    let mask = line.chars().scan(0, |sum, c| {
        *sum += match c {
            '[' => 1,
            ']' => -1,
            _ => 0,
        };
        Some(*sum)
    });

    line.chars()
        .zip(mask)
        .filter_map(|(c, dep)| c.to_digit(10).map(|d| (d as u8, (dep - 1) as u8)))
        .collect()
}

// in a nutshell, if depth of a pair is == 4
// then add left elem to the number on the left of pair,
// right to the the number on the right of pair
// and replace pair with 0
fn explode(pos: usize, v: &mut Vec<(u8, u8)>) {
    // rust is weird and picky about borrowing
    let expl_l = v.get(pos).to_owned().unwrap().0;
    let expl_r = v.get(pos + 1).to_owned().unwrap().0;
    if 0 < pos {
        v[pos - 1].0 += expl_l;
    }
    if pos + 2 < v.len() {
        v[pos + 2].0 += expl_r;
    }
    v.remove(pos);
    v[pos] = (0, 3);
}

// if a number is > 10 replace with a pair of n/2 and n - n/2
fn split(pos: usize, v: &mut Vec<(u8, u8)>) {
    let (val, dep) = v[pos];
    v[pos] = (val / 2, dep + 1);
    v.insert(pos + 1, ((val + 1) / 2, dep + 1));
}

// until no changes occur, explode then split the numbers
fn add_snail(a: Vec<(u8, u8)>, b: Vec<(u8, u8)>) -> Vec<(u8, u8)> {
    // concating two vecs
    let mut v: Vec<(u8, u8)> = a
        .into_iter()
        .chain(b.into_iter())
        .map(|(n, d)| (n, d + 1))
        .collect();
    loop {
        // order of operations screwed me up
        // it's explosions over split, and not call split after explosion
        if let Some(pos) = v.iter().position(|(_, dep)| *dep == 4) {
            explode(pos, &mut v);
        } else if let Some(pos) = v.iter().position(|(val, _)| *val > 9) {
            split(pos, &mut v);
        } else {
            break;
        }
    }
    v
}

// fn printable(v: &[(u8, u8)]) -> String {
//     format!("{:?}", v.iter().map(|(n, _)| *n).collect::<Vec<u8>>())
// }

fn magni(a: Vec<(u8, u8)>) -> u32 {
    // parse vec to correct type
    let mut v: Vec<_> = a.into_iter().map(|(x, y)| (x as u32, y)).collect();
    fn magni_rec(v: &mut Vec<(u32, u8)>) -> u32 {
        if v.len() == 2 {
            return v[0].0 * 3 + v[1].0 * 2;
        }
        // find first pair of nums
        let pos = v
            .windows(2)
            .position(|comp| comp[0].1 == comp[1].1)
            .unwrap();
        // ..[1,9].. how it looks
        // ..(1, x), (9,x).. how it's stored
        // ..(1*3 + 9*2 , x-1).. what i return
        // .. 21 .. after magni
        let mag = (v[pos].0 * 3 + v[pos + 1].0 * 2, v[pos].1 - 1);
        v.remove(pos);
        v[pos] = mag;
        magni_rec(v)
    }
    magni_rec(&mut v)
}

fn part_one(name: &str) -> u32 {
    let data = read_file(name);

    let s = data
        .into_iter()
        .map(parse_line)
        .reduce(add_snail)
        .unwrap();
    // println!("{}", printable(&s));

    magni(s)
}

fn part_two(name: &str) -> u32 {
    use itertools::Itertools;

    let data = read_file(name);
    // find the biggest magnitude of all sums of pairs
    // had to use to_vec since add_snail doesn't borrow
    data
        .into_iter()
        .map(parse_line)
        .permutations(2)
        .map(|pair| add_snail(pair[0].clone(), pair[1].clone()))
        .map(magni)
        .max()
        .unwrap()
}

fn main() {
    let test1 = parse_line("[[[[4,3],4],4],[7,[[8,4],9]]]".to_string());
    let test2 = parse_line("[1,1]".to_string());
    let sol1 = parse_line("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]".to_string());
    assert_eq!(sol1, add_snail(test1, test2));

    assert_eq!(magni(parse_line("[1,9]".to_string())), 21);
    assert_eq!(magni(parse_line("[[9,1],[1,9]]".to_string())), 129);
    assert_eq!(magni(parse_line("[[1,2],[[3,4],5]]".to_string())), 143);
    assert_eq!(part_one("example3.txt"), 4140);
    assert_eq!(part_two("example3.txt"), 3993);
    println!("sol part 1: {}", part_one("input.txt"));
    println!("sol part 2: {}", part_two("input.txt"));
}
