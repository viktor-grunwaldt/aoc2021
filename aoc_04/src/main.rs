fn read_file(name: &str) -> Vec<String> {
    std::fs::read_to_string(name)
        .expect("file not found!")
        .lines()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn make_boards(input: Vec<String>) -> (Vec<i8>, Vec<Vec<Vec<i8>>>) {
    // parse draw numbers
    let draws: Vec<i8> = input[0]
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    //parse into bingo blocks
    

    //parse bingo blocks into bingo boards
    let b: Vec<Vec<Vec<i8>>> = input
        .split(|w| w.is_empty())
        .skip(1)
        .map(|w| w.to_vec())
        .map(|u| {          // each board
            u.iter()
                .map(|w| {  // each row
                    w.split_whitespace() // each "number"
                        .map(|x| x.trim().parse::<i8>().unwrap())
                        .collect()
                })
                .collect()
        })
        .collect();

    (draws, b)
}

fn mark_number(b: &mut [Vec<i8>], mark: i8) {
    b.iter_mut().flatten().for_each(|i| {
        if *i == mark {
            *i = -1;
            
        }
    })
}

fn is_solved(b: &[Vec<i8>]) -> bool {
    //check rows
    // if in any row all numbers are marked
    let rows = b.iter().any(|x| x.iter().all(|&y| y < 0));

    // bit more elegant
    let col = (0..b.len()).any(|i| (0..b[0].len()).map(|j| b[j][i]).all(|x| x < 0));

    rows || col
}

fn calc_sol(b: &[Vec<i8>]) -> u32 {
    // sum of all positive elements
    let res:u32 = b
        .iter()
        .flatten()
        .filter(|&&x| x>0)
        .map(|&x| x as u32)
        .sum();

    res
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
    //read
    let (draws, mut boards) = make_boards(read_file(file));
    //store solved boards
    let mut sol_boards = vec![false; boards.len()];
    // don't run twice
    let mut flag = false;
    // store last board nr
    let mut winner_board: usize = 0;

    for num in draws {
        for (i, board) in boards.iter_mut().enumerate() {
            // check not solved boards
            if !sol_boards.get(i).unwrap() {
                mark_number(board, num);
                if is_solved(board) {
                    sol_boards[i] = true;
                }
            }
        }
        //1 board remains
        if !flag || ((sol_boards.iter().filter(|x| **x).count() + 1) == boards.len()) {
            //find last bingo
            winner_board = sol_boards.iter().position(|&x| !x).unwrap();
            // to calculate solution we need to keep iterating until we hit bingo
            flag = true;
        }
        //0 boards remain
        if flag && (sol_boards.iter().filter(|x| **x).count() == boards.len()) {
            return calc_sol(boards.get(winner_board).unwrap()) * num as u32;
        }
    }

    0
}

fn main() {
    assert_eq!(4512, part_one("example.txt"));
    println!("{}\n", part_one("input.txt"));

    assert_eq!(1924, part_two("example.txt"));
    println!("{}", part_two("input.txt"));
}
