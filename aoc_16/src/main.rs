fn read_file(name: &str) -> Vec<Vec<bool>> {
    std::fs::read_to_string(name)
        .expect("file not found!")
        .lines()
        .map(hex_to_bits)
        .collect()
}

fn hex_to_bits(s: &str) -> Vec<bool> {
    s.chars()
        .flat_map(char_to_bitstream)
        .collect()
}

fn char_to_bitstream(c: char) -> [bool; 4] {
    // each number has to be 4 in len
    // let s = 32- n.leading_zeros() as usize;
    let mut v = [false; 4];
    let mut n = c.to_digit(16).unwrap();
    for i in v.iter_mut().rev() {
        *i = (n & 1) == 1;
        n >>= 1; // haskell kek
    }
    v
}

fn parse_literal(p:&mut Vec<bool>) -> u64 {
    // len of lit = n leading 1's + last 5
    let len = p.iter().enumerate().step_by(5).find(|(_, &x)| !x).unwrap().0 +5;
    // println!("{:?}", bits_to_string(p.get(..len).unwrap()) );
    p.drain(..len).collect::<Vec<bool>>()
        .chunks_exact(5)
        .flat_map(|ch| ch.get(1..).unwrap())
        .fold(0, |acc, &i| (acc << 1) + i as u64)
}

fn parse_subpackets(q: &mut Vec<bool>) -> Vec<u64> {
    let is_len_pck_nr = q.drain(0..1).next().unwrap();
    // If the length type ID is 0, then the next 15 bits are a number that represents 
    // the total length in bits of the sub-packets contained by this packet.
    // If the length type ID is 1, then the next 11 bits are a number that represents 
    // the number of sub-packets immediately contained by this packet.
    let read_len_val = if is_len_pck_nr { 11 } else { 15 };
    let mut len_val = q
        .drain(0..read_len_val)
        .fold(0, |acc, i| (acc << 1) + i as i32);
    
    let mut v: Vec<u64> = vec![];
    while len_val > 0 {
        let old_len = q.len() as i32;
        v.push(parse_packet(q));
        if is_len_pck_nr {
            len_val -=1;
        }
        else {
            len_val -= old_len - q.len() as i32;
        }
    }
    v
}

fn parse_packet(q:&mut Vec<bool>) -> u64 {
    let _version = q.drain(0..3).fold(0, |acc, i| (acc << 1) + i as u32);
    let type_id = q.drain(0..3).fold(0, |acc, i| (acc << 1) + i as u32);

    if type_id == 4 {
        parse_literal(q)
    }
    else {
        let v = parse_subpackets(q);
        match type_id {
        // 4 => parse_literal(q),
        0 => v.iter().sum(),
        1 => v.iter().product(),
        2 => *v.iter().min().unwrap(),
        3 => *v.iter().max().unwrap(),
        5 => if v[0] >  v[1] {1} else {0}
        6 => if v[0] <  v[1] {1} else {0}
        7 => if v[0] == v[1] {1} else {0}
        _ => panic!()
    }}
}

// fn print_bits(v: &[bool]) {
//     println!("{}", v.iter().map(|&b| if b { "1" } else { "0" }).collect::<String>())
// }

// please don't look at part 1, I've hacked a solution from half baked pt 2 code
fn part_one(name: &str) -> u32 {
    
    fn fake_lit(p:&mut Vec<bool>) -> u32 {
        // len of lit = n leading 1's + last 5
        let len = p.iter().enumerate().step_by(5).find(|(_, &x)| !x).unwrap().0 +5;
        let _lit:Vec<bool> = p.drain(..len).collect();
        0
    }
    
    fn parse_operator1(q: &mut Vec<bool>) -> u32 {
        let is_len_in_packets = q.drain(0..1).next().unwrap();
        let read_len_val = if is_len_in_packets { 11 } else { 15 };
        let mut sum = 0;
        let mut len_val = q
            .drain(0..read_len_val)
            .fold(0, |acc, i| (acc << 1) + i as i32);
        
        while len_val > 0 {
            let old_len = q.len() as i32;
            sum += parse_packet1(q);
            if is_len_in_packets {
                len_val -=1;
            }
            else {
                len_val -= old_len - q.len() as i32;
            }
        }
        sum
    }
    
    fn parse_packet1(q:&mut Vec<bool>) -> u32 {
        let version = q.drain(0..3).fold(0, |acc, i| (acc << 1) + i as u32);
        let type_id = q.drain(0..3).fold(0, |acc, i| (acc << 1) + i as u32);
        match type_id {
            4 => version + fake_lit(q),
            _ => version + parse_operator1(q),
        }
    }

    let mut input = read_file(name);
    let ex = vec![
        "8A004A801A8002F478",
        "620080001611562C8802118E34",
        "C0015000016115A2E0802F182340",
        "A0016C880162017C3686B18A3D4780"
    ];
    let an = vec![16, 12, 23, 31];
    // bits_to_string(& hex_to_bits(ex[1]));
    parse_packet1(&mut hex_to_bits(ex[1]));
    for i in 0..4 {
        assert_eq!(parse_packet1(&mut hex_to_bits(ex[i])), an[i]);
    }

    input.iter_mut().map(parse_packet1).sum()
}

fn part_two(name: &str) -> u64 {
    let mut input = read_file(name);

    let ex = vec![
        "C200B40A82",
        "04005AC33890",
        "880086C3E88112",
        "CE00C43D881120",
        "D8005AC2A8F0",
        "F600BC2D8F",
        "9C005AC2F8F0",
        "9C0141080250320F1802104A08"
    ];
    let an = vec![3, 54, 7, 9, 1, 0, 0, 1];

    for i in 0..8 {
        assert_eq!(parse_packet(&mut hex_to_bits(ex[i])), an[i])
    }
    parse_packet(input.iter_mut().next().unwrap())
}
fn main() {

    let ans1 = part_one("input.txt");
    let ans2 = part_two("input.txt");

    println!("pt1: {}\npt2: {}", ans1, ans2);
}
