// problem at:      https://open.kattis.com/problems/bank
// submitted to:    https://open.kattis.com/submissions/5424135

use std::io;

fn next_parse(iter: &mut std::str::SplitWhitespace) -> usize {
    iter.next().unwrap().parse::<usize>().unwrap()
}

fn insert_push(queue: &mut std::vec::Vec::<usize>, money: usize, max_index: usize) {
    let mut temp_value = money;

    for index in (0..(max_index + 1)).rev() {
        if temp_value == 0 {
            break;
        }
        if queue[index] < temp_value {
            let temp_temp = queue[index];
            queue[index] = temp_value;
            temp_value = temp_temp;
        }
    }
}

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line)
        .expect("Could not read line");
    let mut splitted_line = line.trim().split_whitespace();
    let number_of_people = next_parse(&mut splitted_line);
    let total_time = next_parse(&mut splitted_line);
    let mut queue = vec![0; total_time];

    for _ in 0..number_of_people {
        let mut line = String::new();
        io::stdin().read_line(&mut line)
            .expect("Could not read line");
        let mut splitted_line = line.trim().split_whitespace();
        let money_amount = next_parse(&mut splitted_line);
        let time_left = next_parse(&mut splitted_line);
        insert_push(&mut queue, money_amount, time_left);
    }

    let sum : usize = queue
        .iter()
        .sum();

    println!("{}", sum);
}