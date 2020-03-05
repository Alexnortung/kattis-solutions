//problem at:       https://open.kattis.com/problems/pathtracing
//submitted to:     https://open.kattis.com/submissions/5425905

use std::io;
use std::vec;
use std::cmp;

fn main() {
    let mut positions: Vec<(i16, i16)> = Vec::with_capacity(500);
    let mut current_x = 0i16;
    let mut current_y = 0i16;

    let mut max_x = current_x;
    let mut min_x = current_x;
    let mut max_y = current_y;
    let mut min_y = current_y;
    loop {
        let mut line = String::new();
        let read_bytes = io::stdin().read_line(&mut line)
            .expect("could not read line");
        if read_bytes == 0 {
            break;
        }
        match line.trim() {
            "left" => {
                current_x -= 1;
                min_x = cmp::min(current_x, min_x);
            },
            "up" => {
                current_y -= 1;
                min_y = cmp::min(current_y, min_y);
            },
            "right" => {
                current_x += 1;
                max_x = cmp::max(current_x, max_x);
            },
            "down" => {
                current_y += 1;
                max_y = cmp::max(current_y, max_y);
            },
            _ => break,
        }

        // println!("x:{} y:{}", current_x, current_y)

        positions.push((current_x, current_y));
    }

    enum PositionType {
        Empty = 0,
        Traced = 1,
        Start = 2,
        End = 3,
    }

    let size_x = (max_x - min_x + 1) as usize;
    let size_y = (max_y - min_y + 1) as usize;
    // print!("size x:{} y:{}", size_x, size_y);
    let mut map = vec![vec![&PositionType::Empty; size_y]; size_x];
    for position in positions {
        let (x,y) = position;
        let x = x - min_x;
        let y = y - min_y;
        map[x as usize][y as usize] = &PositionType::Traced;
    }

    map[(-min_x) as usize][(-min_y) as usize] = &PositionType::Start;
    map[(current_x - min_x) as usize][(current_y - min_y) as usize] = &PositionType::End;

    let top_bot_border = String::from("#").repeat(size_x + 2);

    //draw
    println!("{}", top_bot_border);
    for y in 0..size_y {
        print!("#");
        for x in 0..size_x {
            let print_char = match map[x][y] {
                PositionType::Empty => ' ',
                PositionType::Traced => '*',
                PositionType::Start => 'S',
                PositionType::End => 'E',
            };
            print!("{}", print_char);
        }
        println!("#");
    }
    println!("{}", top_bot_border);
}