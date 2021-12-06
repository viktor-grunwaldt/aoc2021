use std::cmp;

fn read_file(name: &str) -> Vec<String> {
    std::fs::read_to_string(name)
        .expect("file not found!")
        .lines()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn print_board(b: &Vec<Vec<i8>>) {
    b.iter().for_each(|w| println!("{:?}", w));
    print!("---------------------\n");
}

fn make_boards(input: Vec<String>) -> (Vec<i8>, Vec<Vec<Vec<i8>>>) {
    // parse draw numbers
    let draws: Vec<i8> = input[0]
        .trim()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    //parse into bingo blocks
    let bs: Vec<Vec<String>> = input
        .split(|w| w.is_empty())
        .skip(1)
        .map(|w| w.to_vec())
        .collect();
    // bs.iter().for_each(|w| println!("{:?}", w));
    // println!("{:?}", bs);
    //parse bingo blocks into bingo boards
    let b: Vec<Vec<Vec<i8>>> = bs
        .into_iter()
        .map(|u| {
            // each board
            u.iter()
                .map(|w| {
                    //each row
                    w.split_whitespace() // each "number"
                        .map(|x| x.trim().parse::<i8>().unwrap())
                        .collect()
                })
                .collect()
        })
        .collect();

    // b.iter().for_each(|w| print_board(&w));

    (draws, b)
}

fn mark_number(b: &mut Vec<Vec<i8>>, mark: i8) {
    for i in b {
        for j in i {
            if *j == mark {
                *j = -1;
                return;
            }
        }
    }
}

fn is_solved(b: &Vec<Vec<i8>>) -> bool {
    //check rows
    // if in any row all numbers are marked
    let rows: bool = b.iter().fold(false, |acc, x| {
        acc || x.iter().fold(true, |acc2, y| acc2 && (*y < 0))
    });

    //no idea how to do it for columns lmao
    let mut col = false;
    for i in 0..b.len() {
        let mut acc = true;
        for j in 0..b.get(0).unwrap().len() {
            if !(b[j][i] < 0) {
                acc = false;
                break;
            }
        }
        if acc {
            col = true;
            break;
        }
    }
    if rows {
        println!("row is solved");
        print_board(b);
    }
    if col {
        println!("col is solved");
        print_board(b);
    }
    rows || col
}

fn calc_sol(b: &Vec<Vec<i8>>) -> u32 {
    let res = b
        .iter()
        .flatten()
        .fold(0, |acc, x| acc + cmp::max((*x) as i32, 0));
    res as u32
}

fn part_one(file: &str) -> u32 {
    let (draws, mut boards) = make_boards(read_file(file));

    for num in draws {
        for board in boards.iter_mut() {
            mark_number(board, num);

            if is_solved(board) {
                return calc_sol(board) * (num as u32);
            }
        }
    }

    0
}

fn part_two(file: &str) -> u32 {
    let (draws, mut boards) = make_boards(read_file(file));
    let mut sol_boards = vec![false; boards.len()];
    let mut last_board: usize = 0;
    let mut flag = false;
    let mut winner_board:usize = 0;
    for num in draws {
        for (i, board) in boards.iter_mut().enumerate() {
            // check not solved boards
            if !sol_boards.get(i).unwrap() {
                mark_number(board, num);

                if is_solved(board) {
                    sol_boards[i] = true;
                    last_board = i;
                }
            }
        }
        //1 board remains
        if !flag || ((sol_boards.iter().filter(|x| **x).count() + 1) == boards.len())  {
            //find last bingo
            winner_board = sol_boards.iter().position(|&x| !x).unwrap();
            // to calculate solution we need to keep iterating until we hit bingo
            flag = true;
        }
        //0 boards remain
        if flag && (sol_boards.iter().filter(|x| **x).count() == boards.len()) {
            return calc_sol(boards.get(winner_board).unwrap()) * num as u32
        }
    }

    0
}

fn main() {
    // let example = vec![(1..6).collect(); 5];
    // let example2 = vec![(-1..4).collect(); 5];
    // assert_eq!(false, is_solved(&example));
    // assert_eq!(true, is_solved(&example2));
    // assert_eq!(4512, part_one("example.txt"));
    // println!("{}", part_one("input.txt"));

    assert_eq!(1924, part_two("example.txt"));
    println!("{}", part_two("input.txt"));
}
