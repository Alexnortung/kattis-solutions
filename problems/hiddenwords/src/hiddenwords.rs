use std::io;
use std::vec;

const MAXDEPTH: usize = 10;

#[derive(Copy, Clone)]
struct Char {
    character: char,
    visited: bool,
}

fn recurse_path(mut board: &mut Vec<Vec<Char>>, chars_left: &[char], position: (usize, usize), depth: usize) -> bool {
    if chars_left.len() <= 0 || depth == MAXDEPTH + 1 {
        return true;
    }
    let (x,y) = position;
    let width = board.len();
    let height = board[x].len();
    
    board[x][y].visited = true;

    // println!("x:{} y:{} left:{:?}", x, y, chars_left);

    if x > 0 && 
        board[x-1][y].character == chars_left[0] && 
        !board[x-1][y].visited && 
        recurse_path(&mut board, &chars_left[1..], (x-1, y), depth+1) {
            board[x][y].visited = false;
            return true;
    }

    if y > 0 && 
        board[x][y-1].character == chars_left[0] && 
        !board[x][y-1].visited && 
        recurse_path(&mut board, &chars_left[1..], (x, y-1), depth+1) {
            board[x][y].visited = false;
            return true;
    }

    if x < width - 1 && 
        board[x+1][y].character == chars_left[0] && 
        !board[x+1][y].visited && 
        recurse_path(&mut board, &chars_left[1..], (x+1, y), depth+1) {
            board[x][y].visited = false;
            return true;
    }

    if y < height - 1 && 
        board[x][y+1].character == chars_left[0] && 
        !board[x][y+1].visited && 
        recurse_path(&mut board, &chars_left[1..], (x, y+1), depth+1) {
            board[x][y].visited = false;
            return true;
    }

    board[x][y].visited = false;
    return false;

}

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line)
        .expect("could not read line");
    let mut splitted_line = line.trim().split_whitespace();
    let height = splitted_line.next().unwrap().parse::<usize>().unwrap();
    let width = splitted_line.next().unwrap().parse::<usize>().unwrap();

    let mut board = vec![Vec::with_capacity(height); width];


    for _y in 0..height {
        let mut line = String::new();
        io::stdin().read_line(&mut line)
            .expect("could not read line");
        for (x, c) in line.trim().chars().enumerate() {
            // println!("x:{}", x);

            let c_char = Char {
                character: c,
                visited: false,
            };
            board[x].push(c_char);
        }
    }

    let mut line = String::new();
    io::stdin().read_line(&mut line)
        .expect("could not read line");
    let words = line.trim().parse::<usize>().unwrap();

    let mut word_count = 0;

    for _ in 0..words {
        let mut line = String::new();
        io::stdin().read_line(&mut line)
            .expect("could not read line");

        let chars: Vec<char> = line.trim().chars().collect();
        let first_char: char = chars[0];
        let mut found = false;
        for x in 0..width {
            for y in 0..height {
                // println!("x:{} y:{}", x, y);

                if first_char == board[x][y].character {
                    found = recurse_path(&mut board, &chars[1..], (x,y), 0);
                }
                if found {
                    break;
                }
            }
            if found {
                break;
            }
        }
        if found {
            word_count += 1;
        }
    }

    println!("{}", word_count);
}