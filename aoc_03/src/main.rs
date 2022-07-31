// yeet loading characters C style
fn read_file(name: &str) -> Vec<Vec<u8>> {
    std::fs::read_to_string(name)
        .expect("file not found!")
        .lines()
        // .map(|x| x.parse().expect("sanitize your data!"))  use if you need string
        .map(|x| x.as_bytes().to_vec()) //it's much easier to cast Vec<u8> to str than Vec<char>
        .collect()
}

fn transpose2<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn count_char(pattern: u8, v: Vec<Vec<u8>>) -> Vec<u32> {
    transpose2(v)
        .iter()
        .map(|w| w.iter().filter(|i| **i == pattern).count() as u32)
        .collect()
}

fn vec_u8_to_num(v: Vec<u8>) -> u32 {
    u32::from_str_radix(std::str::from_utf8(&v).unwrap(), 2).unwrap()
}

// I needed to unga bunga convert types
// 48 => '0'
// 49 => '1'
fn part_one(name: &str) -> u32 {
    // reads file as 2d char vector
    let input = read_file(name);
    let size = input.len() as u32;
    // counts 1's in all columns
    let count_bits: Vec<u32> = count_char(49, input);
    // creates vector of most common bits
    let mcb: Vec<u8> = count_bits
        .iter()
        .map(|w| if 2 * w < size { 48 } else { 49 })
        .collect();

    //creates Vec<u8> of least common bits
    let lcb: Vec<u8> = mcb.iter().map(|w| if *w == 49 { 48 } else { 49 }).collect();

    // converts vec<u8> to &str to a number
    let gamma = vec_u8_to_num(mcb);
    let epsilon = vec_u8_to_num(lcb);

    gamma * epsilon
}

fn bin_search(v: &[Vec<u8>], is_o2: bool) -> Result<usize, &'static str> {
    if v.is_empty() {
        return Err("vector is empty");
    }
    let mut l = 0_usize;
    let mut r = v[0].len();
    let len = v[0].len();
    for bits in v.iter().take(len) {
        // o2 found
        if (l + 1) >= r {
            break;
        }
        // step 1: determine if there is more 0's or 1's
        // there is x words, where x = r - l
        // if word on position [x/2] contains 0
        // there is more or equal 0's than 1's
        // if there is the same amount of 1's and 0's then
        // if it's o2 level -> 1
        // if it's co2 level -> 0

        // step 2: shrink the boundaries
        // now if the bit is 0, the right side shrinks
        //     if the bit is 1, the left  side shrinks

        // println!("{:?}\nelem:\t{:?}", bits, (bits[ (l+r+1)/2 ], (l+r)/2));
        if (bits[(l + r) / 2] == 48) == is_o2 {
            // there is more 0's
            // right side shrinks
            // find first 0 bit from right side

            for ith_word in (l..r).rev() {
                if bits[ith_word] == 48 {
                    r = ith_word + 1;
                    break;
                }
            }
        } else {
            // there is more 1's
            // left side shrinks
            // find first 1 bit from left side
            for (ith_word, ch) in bits.iter().enumerate().take(r).skip(l) {
                if *ch == 49 {
                    l = ith_word;
                    break;
                }
            }
        }
    }
    Ok(l)
}

fn part_two(name: &str) -> u32 {
    let mut input = read_file(name);
    input.sort_unstable();

    // easier to search rows than columns
    let bin_tr = transpose2(input.clone());

    //finds most/least used bits
    let pos_o2 = bin_search(&bin_tr, true).unwrap();
    let pos_co2 = bin_search(&bin_tr, false).unwrap();

    // converts vec<u8> to &str to a number
    let o2 = vec_u8_to_num(input[pos_o2].clone());
    let co2 = vec_u8_to_num(input[pos_co2].clone());

    o2 * co2
}

fn main() {
    assert_eq!(198, part_one("example.txt"));
    println!("{}", part_one("input.txt"));

    assert_eq!(230, part_two("example.txt"));
    println!("{}", part_two("input.txt"));
}
