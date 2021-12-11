use std::collections::HashSet;

fn read_file(name: &str) -> Vec<Vec<u8>> {
    std::fs::read_to_string(name)
        .expect("file not found!")
        .lines()
        .map(|x| x.chars().map(|y| y as u8 - 48).collect())
        .collect()
}

const MAX:usize = 10;

fn check_neigh(x:usize, y:usize) -> Vec<(usize,usize)> {
    let mut n: Vec<(usize,usize)> = vec![];
    for i in (x-1)..(x + 2) {
        for j in (y-1)..(y+2) {
            if i != x || j != y {
                n.push((i,j));
            }
        }
    }
    n
}

fn part_one(name: &str, iter:u32) -> u64 {
    let mut tab = read_file(name);

    //padding cuz fuck border checking
    tab.insert(0, vec![99;MAX]);
    tab.push(vec![99;MAX]);
    for line in tab.iter_mut() {
        line.insert(0, 99);
        line.push(99);
    }

    let mut sum = 0u64;
    for _ in 0..iter {
        // inc ene lvl
        for i in 1..11 {
            for j in 1..11 {
                tab[i][j]+=1;
            }
        }
        // check until no expl found
        let mut flag = true;
        let mut exploded:HashSet<(usize, usize)> = HashSet::new();
        while flag {
            let mut expl:u32 = 0;

            for i in 1..11 {
                for j in 1..11 {
                    if tab[i][j] >= 10 && !exploded.contains(&(i,j)) { //expl found
                        expl+=1;
                        exploded.insert((i,j)); // expl happens only 1 time
                        for (x, y) in check_neigh(i, j) {
                            if tab[x][y] < 10 {
                                tab[x][y] += 1;
                            }
                        }
                    }
                }
            }
            if expl == 0 {
                flag = false;
            }
        }
        sum += exploded.len() as u64;
        for (i, j) in exploded {
            tab[i][j] = 0;
        }
    }
    
    sum
}

fn part_two(name: &str) -> u64 {
    let mut tab = read_file(name);

    //padding cuz fuck border checking
    tab.insert(0, vec![99;MAX]);
    tab.push(vec![99;MAX]);
    for line in tab.iter_mut() {
        line.insert(0, 99);
        line.push(99);
    }
    // neat infinite counting loop
    for i in 1.. { 
        // inc ene lvl
        for i in 1..11 {
            for j in 1..11 {
                tab[i][j]+=1;
            }
        }
        let mut flag = true;
        let mut exploded:HashSet<(usize, usize)> = HashSet::new();
        // check until no expl found
        while flag {
            let mut expl:u32 = 0;

            for i in 1..11 {
                for j in 1..11 {
                    if tab[i][j] >= 10 && !exploded.contains(&(i,j)) { //expl found
                        expl+=1;
                        exploded.insert((i,j)); // expl happens only 1 time
                        for (x, y) in check_neigh(i, j) {
                            if tab[x][y] < 10 {
                                tab[x][y] += 1;
                            }
                        }
                    }
                }
            }
            if expl == 0 { // stop when no more expl
                flag = false;
            }
        }
        if exploded.len() == 100 { // end if all expl
            return i;
        }
        for (i, j) in exploded {
            tab[i][j] = 0;
        }
    }
    
    0 // should never happen but rust is picky
}

fn main() {

    assert_eq!(1656,part_one("example.txt", 100));
    println!("{}",part_one("input.txt", 100));
    
    
    assert_eq!(195,part_two("example.txt"));
    println!("{}",part_two("input.txt"));
}
