use itertools::sorted;
use std::collections::{HashSet, LinkedList};

fn read_file(name: &str) -> Vec<Vec<u8>> {
    std::fs::read_to_string(name)
        .expect("file not found!")
        .lines()
        .map(|x| x.chars().map(|y| y as u8 - 48).collect())
        .collect()
}

fn part_one(name: &str) -> u32 {
    let input = read_file(name);
    // wrap input with 9's
    let len = input[0].len() + 2;
    let size = input.len() + 2;
    let mut tab = vec![vec![9u8; len]; size];
    for i in 0..(size - 2) {
        for j in 0..(len - 2) {
            tab[i + 1][j + 1] = input[i][j];
        }
    }
    let mut sum: u32 = 0;
    for i in 1..(size - 1) {
        for j in 0..(len - 1) {
            if tab[i][j] < tab[i - 1][j]
                && tab[i][j] < tab[i + 1][j]
                && tab[i][j] < tab[i][j - 1]
                && tab[i][j] < tab[i][j + 1]
            {
                sum += 1 + tab[i][j] as u32;
            }
        }
    }
    sum
}

fn part_two(name: &str) -> u32 {
    let input = read_file(name);
    // wrap input with 9's
    let len = input[0].len() + 2;
    let size = input.len() + 2;
    let mut tab = vec![vec![9u8; len]; size];
    // add borders for no OutOfBounds
    for i in 0..(size - 2) {
        for j in 0..(len - 2) {
            tab[i + 1][j + 1] = input[i][j];
        }
    }
    let mut pool_seeds: LinkedList<(usize, usize)> = LinkedList::new();
    // find seeds
    for i in 1..(size - 1) {
        for j in 0..(len - 1) {
            if tab[i][j] < tab[i - 1][j]
                && tab[i][j] < tab[i + 1][j]
                && tab[i][j] < tab[i][j - 1]
                && tab[i][j] < tab[i][j + 1]
            {
                pool_seeds.push_back((i, j));
            }
        }
    }

    let mut pools: Vec<HashSet<(usize, usize)>> = Vec::new();
    //bfs pools
    for (i, &seed) in pool_seeds.iter().enumerate() {
        let mut to_visit: LinkedList<(usize, usize)> = LinkedList::new();
        to_visit.push_back(seed);
        pools.push(HashSet::new());
        while !to_visit.is_empty() {
            let c = to_visit.pop_front().unwrap();
            pools[i].insert(c);
            for t in [
                (c.0 - 1, c.1),
                (c.0 + 1, c.1),
                (c.0, c.1 - 1),
                (c.0, c.1 + 1),
            ] {
                if !pools[i].contains(&t) && tab[t.0][t.1] != 9 {
                    to_visit.push_back(t);
                }
            }
        }
    }
    let sol: u32 = sorted(pools.iter().map(|x| x.len() as u32))
        .rev()
        .take(3)
        .product();
    sol
}

fn main() {
    assert_eq!(15, part_one("example.txt"));
    println!("{}", part_one("input.txt"));

    assert_eq!(1134, part_two("example.txt"));
    println!("{}", part_two("input.txt"));
}
