use std::io;
use std::cmp;

fn parse_next(iter: &mut std::str::SplitWhitespace) -> i32 {
    return iter.next().unwrap().parse::<i32>().unwrap();
}

fn main() {
    let mut line = String::with_capacity(3);
    io::stdin().read_line(&mut line)
        .expect("Could not read line");
    let num_days = line.trim().parse::<i32>().unwrap();
    let mut line = String::new();
    io::stdin().read_line(&mut line)
        .expect("Could not read line");
    let mut splitted_line = line.trim().split_whitespace();
    let mut start_day = parse_next(&mut splitted_line);
    let mut middle_day = parse_next(&mut splitted_line);
    let mut d = 1;
    let mut t = 40;
    for i in 0..(num_days - 2) {
        let last_day = parse_next(&mut splitted_line);
        let max = cmp::max(start_day, last_day);
        if max < t {
            t = max;
            d = i + 1;
        }
        start_day = middle_day;
        middle_day = last_day;
    }
    println!("{} {}", d, t);
}