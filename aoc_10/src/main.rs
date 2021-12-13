use itertools::sorted;
use std::collections::LinkedList;
// I could use hashmaps to make 1 line matches, but why bother?
// if they are inside a function they'd be constructed/destructed with every call (probably)
// I also could use phf::phf_map but I don't want to import a crate to save a few LOC


fn read_file(name: &str) -> Vec<String> {
    std::fs::read_to_string(name)
        .expect("file not found!")
        .lines()
        .map(|x| x.parse().unwrap())
        .collect()
}
const BRACKETS: [[char; 4]; 2] = [['(', '[', '{', '<'], [')', ']', '}', '>']];

fn match_to<T>(bracket:&char,a:T,b:T,c:T,d:T,def:T) -> T {
    match bracket {
        ')' => a,
        ']' => b,
        '}' => c,
        '>' => d,
        _ => def
    }
}

fn check_brackets(line: &str) -> Option<u32> {
    let mut stack: LinkedList<char> = LinkedList::new();
    for ch in line.chars() {
        // opening bracket
        if BRACKETS[0].contains(&ch) {
            stack.push_back(ch);
        }
        // closing bracket
        else {
            //find other bracket
            let ch_match = match_to(&ch, '(', '[', '{', '<', ' ');
            // correct bracket
            if *stack.back().unwrap() == ch_match {
                stack.pop_back();
            }
            //incorrect bracket
            else {
                return Some(match_to(&ch, 3, 57, 1197, 25137, 999_999_999));
            }
        }
    }
    None //don't care if brackets are missing
}


fn part_one(name: &str) -> u32 {
    let input = read_file(name);

    let sum: u32 = input.iter().filter_map(|line| check_brackets(line)).sum();
    sum
}

fn count_missing_brackets(line: &str) -> Option<u64> {
    let mut stack: LinkedList<char> = LinkedList::new();
    for ch in line.chars() {
        // opening bracket
        if BRACKETS[0].contains(&ch) {
            stack.push_back(ch);
        }
        // closing bracket
        else {
            //find other bracket
            let ch_match = match_to(&ch, '(', '[', '{', '<', ' ');
            // correct bracket
            if *stack.back().unwrap() == ch_match {
                stack.pop_back();
            }
            //incorrect bracket
            else {
                //don't care if brackets are corrupted
                return None;
            }
        }
    }

    Some(stack.iter().rev().fold(0, |acc, x| {
        5 * acc
            + match x {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => 0, //shouldn't happen
            }
    }))
}

fn part_two(name: &str) -> u64 {
    let input = read_file(name);

    let sum: Vec<u64> =
        sorted(input.iter().filter_map(|line| count_missing_brackets(line))).collect();

    sum[sum.len() / 2]
}

fn main() {
    let ex1 = "{([(<{}[<>[]}>{[]{[(<()>".to_string();
    let ex2 = "[({(<(())[]>[[{[]{<()<>>".to_string();
    assert_eq!(Some(1197), check_brackets(&ex1));
    assert_eq!(26397, part_one("example.txt"));
    println!("{}", part_one("input.txt"));


    assert_eq!(Some(288957), count_missing_brackets(&ex2));
    assert_eq!(288957, part_two("example.txt"));
    println!("{}", part_two("input.txt"));
}
