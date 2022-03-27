use std::cmp;
use std::cmp::Ordering;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::{BinaryHeap, HashMap};

fn read_file(name: &str) -> Vec<Vec<u32>> {
    std::fs::read_to_string(name)
        .expect("file not found!")
        .lines()
        .map(|x| x.chars().map(|y| (y as u8 - 48) as u32).collect())
        .collect()
}

fn part_one(name: &str) -> u32 {
    let mut tab = read_file(name);
    let size = tab.len();
    let len = tab[0].len();
    //padding cuz fuck border checking
    tab.insert(0, vec![9999; len]); // fst
    tab.push(vec![9999; len]); // last
    tab.iter_mut().for_each(|l| { // middle
        l.insert(0, 9999);
        l.push(9999)
    });
    tab[0][1] = 0;
    tab[1][1] = 0;
    for i in 0..size { //dp
        for j in 0..len {
            tab[i + 1][j + 1] += cmp::min(tab[i][j + 1], tab[i + 1][j]);
        }
    }
    tab[size][len]
}

#[derive(Eq)]
struct Path {
    len: u32,
    node: (usize, usize),
}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        self.len == other.len
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// reversed ordering
impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        other.len.cmp(&self.len)
    }
}

// I've rewritten petgraph's dijkstra impl
fn dj_extra(v: &[Vec<u32>], start: (usize, usize), end: (usize, usize)) -> u32 {
    let mut heap = BinaryHeap::new();
    let mut scores = HashMap::new();
    let mut visited = vec![vec![false; v[0].len()]; v.len()];

    scores.insert(start, 0);
    heap.push(Path {
        len: 0,
        node: start,
    });

    while let Some(cur) = heap.pop() {
        if visited[cur.node.0][cur.node.1] {
            continue;
        }
        if cur.node == end {
            return cur.len; // break;
        }

        for t in [
            (cur.node.0 - 1, cur.node.1),
            (cur.node.0 + 1, cur.node.1),
            (cur.node.0, cur.node.1 - 1),
            (cur.node.0, cur.node.1 + 1),
        ] {
            if v[t.0][t.1] != 9999 && !visited[cur.node.0][cur.node.1] {
                let new_len = cur.len + v[t.0][t.1];
                match scores.entry(t) {
                    Occupied(ent) => {
                        if new_len < *ent.get() {
                            *ent.into_mut() = new_len;
                            heap.push(Path {
                                len: new_len,
                                node: t,
                            });
                        }
                    }
                    Vacant(ent) => {
                        ent.insert(new_len);
                        heap.push(Path {
                            len: new_len,
                            node: t,
                        });
                    }
                }
            }
        }
        visited[cur.node.0][cur.node.1] = true;
    }
    *scores.get(&end).unwrap() // scores
}

fn part_two(name: &str) -> u32 {
    let input = read_file(name);
    let size = input.len();
    let len = input[0].len();

    let mut tab: Vec<Vec<u32>> = vec![vec![0; len * 5]; size * 5];

    for i in 0..5 {
        for j in 0..5 {
            for k in 0..size {
                for l in 0..len {
                    tab[k + i * size][l + j * len] = (input[k][l] + (i + j) as u32  - 1) % 9 + 1;
                }
            }
        }
    }

    // prt(&tab);
    //padding cuz fuck border checking
    tab.insert(0, vec![9999; len * 5]); // fst
    tab.push(vec![9999; len * 5]); // last
    tab.iter_mut().for_each(|l| {
        // middle
        l.insert(0, 9999);
        l.push(9999)
    });

    // dp isn't enough, needs dijkstra

    dj_extra(&tab, (1, 1), (size * 5, len * 5))
}
fn main() {
    assert_eq!(40, part_one("example.txt"));
    println!("{}", part_one("input.txt"));

    // assert_eq!(315, part_two("example.txt"));
    println!("{}", part_two("input.txt")); // 2868
}
