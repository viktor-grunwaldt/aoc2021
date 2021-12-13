use std::collections::HashMap;

fn read_file(name: &str) -> Vec<Vec<String>> {
    std::fs::read_to_string(name)
        .expect("file not found!")
        .lines()
        .map(|x| x.split('-').map(|y| y.parse().unwrap()).collect())
        .collect()
}

#[derive(Debug)]
struct Cave {
    connected: Vec<usize>,
    large: bool,
}

fn count_paths(caves: &[Cave], visited: &mut Vec<bool>, pos: usize, end: usize) -> u32 {
    let cave = &caves[pos];

    if pos == end {
        1
    } else if visited[pos] && !cave.large {
        0
    } else {
        visited[pos] = true;
        let paths = cave
            .connected
            .iter()
            .map(|&new_pos| count_paths(caves, visited, new_pos, end))
            .sum();
        visited[pos] = false;
        paths
    }
}

fn count_paths_2(
    caves: &[Cave],
    visited: &mut Vec<u8>,
    multi_visited: bool,
    pos: usize,
    start: usize,
    end: usize,
) -> u32 {
    let cave = &caves[pos];
    let second_visit = visited[pos] >= 1 && !cave.large;

    if pos == end {
        1
    } else if second_visit && (multi_visited || pos == start) {
        0
    } else {
        let multi_visited = multi_visited || second_visit;
        visited[pos] += 1;
        let paths = cave
            .connected
            .iter()
            .map(|&new_pos| count_paths_2(caves, visited, multi_visited, new_pos, start, end))
            .sum();
        visited[pos] -= 1;
        paths
    }
}

fn sol(name: &str, is_part_one: bool) -> u32 {
    let input = read_file(name);
    let mut indexes: HashMap<String, usize> = HashMap::new();

    let mut caves: Vec<Cave> = Vec::new();

    for edge in input {
        for node in &edge {
            if !indexes.contains_key(node) {
                indexes.insert(node.clone(), caves.len());

                caves.push(Cave {
                    connected: Vec::new(),
                    large: node.chars().any(|c| c.is_ascii_uppercase()),
                });
            }
        }

        let a = indexes[&edge[0]];
        let b = indexes[&edge[1]];
        caves[a].connected.push(b);
        caves[b].connected.push(a);
    }

    let start = indexes["start"];
    let end = indexes["end"];
    if is_part_one {
        count_paths(&caves, &mut vec![false; caves.len()], start, end)
    } else {
        count_paths_2(&caves, &mut vec![0; caves.len()], false, start, start, end)
    }
}

fn main() {
    assert_eq!(10, sol("ex1.txt", true));
    assert_eq!(19, sol("ex2.txt", true));
    assert_eq!(226, sol("ex3.txt", true));

    println!("{}", sol("input.txt", true));

    assert_eq!(36, sol("ex1.txt", false));
    assert_eq!(103, sol("ex2.txt", false));
    assert_eq!(3509, sol("ex3.txt", false));

    println!("{}", sol("input.txt", false));
}
