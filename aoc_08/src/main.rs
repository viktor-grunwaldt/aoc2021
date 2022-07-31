use itertools::Itertools;
use std::collections::HashMap;

fn read_file(name: &str) -> Vec<Vec<String>> {
    std::fs::read_to_string(name)
        .expect("file not found!")
        .lines()    
        .map(|x| 
            x.split_whitespace()
            .map(|y| y.parse().unwrap())
            .collect()
        )
        .collect()
}
fn part_one(name: &str) -> u32 {

    let input = read_file(name);

    // it's easier to use input than try splitting into two tuples
    // [0..9] unique signal patterns |([10] separator)|  [11..14] output digits

    // easy digits are:
    // 1 : len = 2
    // 4 : len = 4
    // 7 : len = 3
    // 8 : len = 7
    let easy_digits:u32 = input.iter().map(|l| 
        l.iter().skip(11).filter(|w| 
            matches!(w.len(), 2|3|4|7)).count() as u32
    ).sum();

    easy_digits
}

const LOWER_A: usize = 'a' as usize;

fn usz2chr(c:usize) -> char{
    if c > 7{
        panic!("tried to convert a non-letter");
    }
    (c + LOWER_A) as u8 as char
}

fn chr2usz(c:char) -> usize{
    if !('a'..='g').contains(&c) {
        panic!("tried to convert a non-letter");
    }
    c as usize - LOWER_A
}

fn diff(a:&str, b:&str) -> char{
    // a should be the bigger set
    assert!(a.len()>b.len());
    // for my usecase, I need only 1 char diff
    let sol: char = a.chars().find(|c| !b.contains(*c)).expect("no diffs found :<");
    sol
}

fn decode_string(a:&str, decoder:&[char]) -> String {
    if a.chars().any(|c|  !('a'..='g').contains(&c)) {
        panic!("invalid string");
    }
    let sol:String = a.chars().map(|c|
         decoder[chr2usz(c)])
         .collect();
    
    sol
}

fn encode_string(a:&str, decoder:&[char]) -> String {
    if a.chars().any(|c|  !('a'..='g').contains(&c)) {
        panic!("invalid string");
    }
    let mut encoder = vec![' ';7];
    // generates a reversed decoder

    decoder
        .iter()
        .enumerate()
        .filter(|(_, &ch)| ch != ' ')
        .for_each(|(i, &x)| encoder[chr2usz(x)] = usz2chr(i));
    
    // I HAVE NO IDEA WHY I NEED TO COPY THIS CODE FROM DECODE
    // BUT ONLY THIS WAY IT WORKS
    let sol:String = a
        .chars()
        .map(|c| encoder[chr2usz(c)])
        .collect();
    
    sol
}

fn part_two(name: &str) -> u64 {
    let digits:Vec<String> = vec![
        String::from("abcefg"), //    0
        String::from("cf"), //        1
        String::from("acdeg"), //     2
        String::from("acdfg"), //     3
        String::from("bcdf"), //      4
        String::from("abdfg"), //     5
        String::from("abdefg"), //    6
        String::from("acf"), //       7
        String::from("abcdefg"), //   8
        String::from("abcdfg"), //    9
    ];
    
    let mut to_digits:HashMap<String, char> = HashMap::new();
    for (i, digit) in digits.iter().enumerate(){
        to_digits.insert(digit.clone(), (i as u8 + 48) as char);
    }

    let input = read_file(name);
    // let signals: Vec<(Vec<String>,Vec<String>)> = 
    //     input.iter().map(|x| (*x).split_off(10)).collect();

    // it's easier to use input than try splitting into two tuples
    // [0..9] unique signal patterns |([10] separator)|  [11..14] output digits
    
    // easy digits are:
    // 1 : len = 2
    // 4 : len = 4
    // 7 : len = 3
    // 8 : len = 7
    
    //   aaaa
    //  b    c
    //  b    c
    //   dddd
    //  e    f
    //  e    f
    //   gggg

    // fun part: decoding digits

    // 8 is useless
    // but if we have 1 7 and 4 we can 
    // implicitly deduce a from 1 and 7
    // with a, we can split numbers into two groups
    // have a: 0 2 3 5 6 7 8 9 don't have a: 1 4
    // aaaaand it's useless, since we already know that

    // take 2: use frequency analysis:
    // a : 8
    // b : 6
    // c : 8
    // d : 7
    // e : 4
    // f : 9
    // g : 7

    // unique are: b e f
    // if we find a  also unique c
    // to find: d  g
    // from 4: d = 4\{b,c,f}
    // g is last not known

    // now as algo:
    // 1. find 1 and 7, determine a = 7\1
    // 2. freq anal, find b e f and c as (not 'a' and 8)
    // 3. find d as 4 \ encode("bcf")
    // 4. find g as last missing
    
    // store letter conversion
    // store encoded numbers 

    let mut sum: u64 = 0;

    for line in input{
        // let mut decode:HashMap<char,char> = HashMap::new();
        // HASHMAPS ARE STUPID
        // so I'm storing my dict as a vector with a-> 0, b->1 etc...
        let mut decoder = vec![' ';7];
        let mut encoded_numbers = vec![String::new(); 10];
        let scrambled:Vec<String> = line.iter().take(10).cloned().collect();
        // 1.
        encoded_numbers[1] = scrambled.iter().find(|x| x.len()==2).cloned().unwrap();
        encoded_numbers[4] = scrambled.iter().find(|x| x.len()==4).cloned().unwrap();
        encoded_numbers[7] = scrambled.iter().find(|x| x.len()==3).cloned().unwrap();
        
        decoder[chr2usz(diff(&encoded_numbers[7], &encoded_numbers[1]))] = 'a';
        // 2.
        let mut letters = vec![0u32;7];
        for num in scrambled {
            num.chars().for_each(|bit| letters[bit as usize - LOWER_A] += 1);
        }
        // find b e f
        for (le, freq) in letters.iter().enumerate(){
            match freq {
                6 => decoder[le] = 'b',
                4 => decoder[le] = 'e',
                9 => decoder[le] = 'f',
                8 => if decoder[le] != 'a' { decoder[le] = 'c'},
                _ => (),
            };
        }
        
        // 3.
        let char_to_d = diff(&encoded_numbers[4], &encode_string("bcf", &decoder));
        decoder[chr2usz(char_to_d)] = 'd';
        // 4.
        for (i, c) in decoder.iter_mut().enumerate() {
            if *c == ' ' {
                decoder[i] = 'g';
                break;
            }
        }
        // sanity check
        if decoder.iter().any(|&x| x == ' ') {
            println!("{:?}", decoder);
            panic!("fuck!, something broke and didn't decode");
        }

        // we can decode numbers now, but they are scrambled
        //                 decode last 4 numbers
        sum += line.iter().skip(11).map(|num| {
            to_digits.get(                     // after decoding segments get number
                &decode_string(num, &decoder) // decode segments
                .chars().sorted()           // sort decoded segments to match our default
                .collect::<String>()        // after sorting segments put in order
            ).unwrap()                      // panic if segment not in dict
        }).collect::<String>().parse::<u64>().unwrap(); // group digits as string and try parsing
    }
    
    sum
}

fn main() {
    assert_eq!(26, part_one("example.txt"));
    assert_eq!('d',diff("abcd", "abc"));
    let decoder:Vec<char> = vec!['e','f',' ',' ',' ',' ',' ',];
    let encoder:Vec<char> = vec![' ',' ',' ',' ','a','b',' ',];
    let baba = "baba".to_string();
    let fefe = "fefe".to_string();

    // decode baba to fefe
    assert_eq!(decode_string(&baba, &decoder), "fefe");
    // decode fefe to baba with encoder
    assert_eq!(decode_string(&fefe, &encoder), "baba");
    // encode fefe with decoder
    assert_eq!(encode_string(&fefe, &decoder), "baba");
    // let enc = encode_string(&"baba".to_string(), &decode);
    // println!("{}", enc);
    
    // assert_eq!(0, part_two("example.txt"));
    
    assert_eq!(61229, part_two("example.txt"));

    println!("{}", part_two("input.txt"));
    
}
