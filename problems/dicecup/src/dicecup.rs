use std::io;
use std::cmp;

fn main() {
    let mut line = String::with_capacity(6);
    io::stdin().read_line(&mut line)
        .expect("Could not read line");
    let mut splitted_line = line.trim().split_whitespace();
    let d1 = splitted_line.next().unwrap().parse::<usize>().unwrap();
    let d2 = splitted_line.next().unwrap().parse::<usize>().unwrap();
    let min = cmp::min(d1, d2);
    let max = cmp::max(d1, d2);
    let numbers_to_print = 1 + max - min;
    for i in (0..numbers_to_print).rev() {
        println!("{}", max + 1 - i);
    }
}