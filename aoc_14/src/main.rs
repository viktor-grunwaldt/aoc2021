use std::collections::HashMap;

fn read_file(name: &str) -> Vec<String> {
    std::fs::read_to_string(name)
        .expect("file not found!")
        .lines()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn part_one(name: &str, iter: usize) -> u64 {
    if iter > 30 {
        panic!("I'm not a supercomputer!");
    }
    let input = read_file(name);
    let mut inp = input.split(|w| w.is_empty());
    let mut word = inp.next().unwrap().first().unwrap().clone();
    let b = word.chars().last().unwrap().to_string(); 
    let p2 = inp.next().unwrap();

    let mut subst: HashMap<String, String> = HashMap::new();

    for rule in p2 {
        let templ: Vec<&str> = rule.split(" -> ").collect();
        subst.insert(
            templ[0].to_string(),
            format!("{}{}", templ[0].split_at(1).0, templ[1]),
        );
    }

    for _ in 0..iter {
        let nword: String = word
            .chars()
            .collect::<Vec<char>>()
            .windows(2)
            .map(|ch| {
                let k = ch.iter().cloned().collect::<String>();
                subst[&k].clone()
            })
            .collect();
        word = nword + &b;
    }
    let letter_counts: HashMap<char, u64> = word.chars().fold(HashMap::new(), |mut map, c| {
        *map.entry(c).or_insert(0) += 1;
        map
    });

    let max = letter_counts.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap();
    let min = letter_counts.iter().min_by(|a, b| a.1.cmp(b.1)).unwrap();
    max.1 - min.1
}

fn part_two(name: &str, iter: usize) -> Option<u64> {
    if iter > 50 {
        panic!("I'm not a supercomputer!");
    }
    let input = read_file(name);
    let mut inp = input.split(|w| w.is_empty());
    let word = inp.next().unwrap().first().unwrap().clone();
    let p2 = inp.next()?;

    let last = word.chars().rev().next()?; //take last letter
    let first = word.chars().next()?; //take first letter

    // rules as hashmap
    let mut rules: HashMap<String, (String, String)> = HashMap::new();
    // count adjacent chars
    let mut count_pairs: HashMap<String, u64> = HashMap::new();
    // fill rules and adj chars
    for rule in p2 {
        // rule: "AB -> C"
        // dict: "(AB , (AC, CB))"
        let (left, right)  = rule.split_once(" -> ")?;
        let (fst, snd) = left.split_at(1);
        rules.insert(
            left.to_string(),
            ([fst, right].concat(),
            [right, snd].concat()),
        );
        count_pairs.insert(left.to_string(), 0);
    }

    // decompose word onto adj hashmap
    for pair in word.chars().collect::<Vec<char>>().windows(2) {
        let k = pair.iter().cloned().collect::<String>();
        count_pairs.entry(k).and_modify(|e| *e += 1);
    }
    for _ in 0..iter {
        // for each pair of adj characters we apply the rule and add the newly created pairs
        for (k, v) in count_pairs.clone().iter() {
            let old_v = *v; // we need to subtract the old value
            let (l, r) = &rules[k];
            count_pairs.entry(l.to_string()).and_modify(|ptr| *ptr += v);
            count_pairs.entry(r.to_string()).and_modify(|ptr| *ptr += v);
            count_pairs.entry(k.to_string()).and_modify(|ptr| *ptr -= old_v);
        }
    }

    // count chars (we add leftmost and rightmost char so that we can divide the count by 2)
    let mut char_count: HashMap<char, u64> = [(first, 1), (last, 1)]
        .iter().cloned().collect();
    for (k, v) in count_pairs.into_iter() {
        for ch in k.chars() {
            char_count
                .entry(ch)
                .and_modify(|ptr| *ptr += v)
                .or_insert(v);
        }
    }
    // find max, min
    let max = char_count.values().max()?;
    let min = char_count.values().min()?;
    Some((max - min)/2)
}

fn main() {
    assert_eq!(1588, part_one("example.txt", 10));
    // println!("{}", part_one("input.txt", 10));
    assert_eq!(1588, part_two("example.txt", 10).unwrap());
    println!("{}", part_two("input.txt", 40).unwrap());
}
