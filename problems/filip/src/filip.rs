use std::io;
use std::cmp;

fn flip(num: &str) -> u16 {
    num.chars()
        .rev()
        .collect::<String>()
        .parse::<u16>()
        .unwrap()
}

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line)
        .expect("Could not read line");
    let splitted_line = line.trim().split_whitespace();
    let max = splitted_line
        .fold(0u16, |acc, x| cmp::max(flip(x), acc));
    println!("{}", max);
}