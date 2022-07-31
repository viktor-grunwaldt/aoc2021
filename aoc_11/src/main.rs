use std::collections::HashSet;

fn read_file(name: &str) -> Vec<Vec<u8>> {
    std::fs::read_to_string(name)
        .expect("file not found!")
        .lines()
        .map(|x| x.chars().map(|y| y as u8 - 48).collect())
        .collect()
}

const MAX: usize = 10;

fn near_index(x: usize, y: usize) -> Vec<(usize, usize)> {
    // i + i/4 generates:
    // 1 2 3 4 6 7 8 9
    (0..8).map(|i| (x - 1 + (i + i / 4) / 3, y - 1 + (i + i / 4) % 3)).collect()
    
}

fn iter_expl(tab: &mut [Vec<u8>]) -> HashSet<(usize, usize)> {
    let mut exploded: HashSet<(usize, usize)> = HashSet::new();
    loop {
        let mut expl: u32 = 0;

        for i in 1..11 {
            for j in 1..11 {
                if tab[i][j] >= 10 && !exploded.contains(&(i, j)) {
                    //expl found
                    expl += 1;
                    // prevent multiple expl at the same place
                    exploded.insert((i, j)); 
                    // increment nearby elems
                    near_index(i, j).into_iter().for_each(|(x,y)| if tab[x][y] < 10 {tab[x][y] += 1});
                }
            }
        }

        if expl == 0 {
            return exploded;
        }
    }
}

fn part_one(name: &str, iter: u32) -> u64 {
    let mut tab = read_file(name);

    //padding cuz fuck border checking
    tab.insert(0, vec![99; MAX]);   // fst
    tab.push(vec![99; MAX]);        // last
    tab.iter_mut().for_each(|l| {   // middle
        l.insert(0, 99);
        l.push(99)
    });

    let mut sum = 0u64;
    for _ in 0..iter {
        // inc ene lvl
        (0..(MAX*MAX)).for_each(|i| tab[i / MAX + 1][i % MAX + 1] += 1);

        // check until no expl found
        let exploded = iter_expl(&mut tab);
        sum += exploded.len() as u64;

        // zero out flashes
        exploded.into_iter().for_each(|(i,j)| tab[i][j] = 0);

    }

    sum
}

fn part_two(name: &str) -> Option<u64> {
    let mut tab = read_file(name);

    //padding cuz fuck border checking
    tab.insert(0, vec![99; MAX]);   // fst
    tab.push(vec![99; MAX]);        // last
    tab.iter_mut().for_each(|l| {   // middle
        l.insert(0, 99);
        l.push(99)
    });

    // neat infinite counting loop
    for day in 1.. {
        // inc energy lvl
        (0..100).for_each(|i| tab[i / MAX + 1][i % MAX + 1] += 1);

        // check until no expl found
        let exploded = iter_expl(&mut tab);

        if exploded.len() == 100 {
            // end if all expl
            return Some(day);
        }
        // zero out flashes
        exploded.into_iter().for_each(|(i,j)| tab[i][j] = 0);
    }

    None // should never happen but rust is picky
}

fn main() {
    assert_eq!(1656, part_one("example.txt", 100));
    println!("{}", part_one("input.txt", 100));

    assert_eq!(Some(195), part_two("example.txt"));
    println!("{}", part_two("input.txt").unwrap());
}
