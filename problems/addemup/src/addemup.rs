use std::io;
use std::collections::HashMap;

fn next_iter(iter: &mut std::str::SplitWhitespace) -> usize {
    return iter.next().unwrap().parse::<usize>().unwrap();
}

fn main() {
    let mut flip_dictionary: HashMap<char, char> = HashMap::new();
    flip_dictionary.insert('0', '0');
    flip_dictionary.insert('1', '1');
    flip_dictionary.insert('2', '2');
    flip_dictionary.insert('5', '5');
    flip_dictionary.insert('6', '9');
    flip_dictionary.insert('8', '8');
    flip_dictionary.insert('9', '6');
    let mut line = String::new();
    io::stdin().read_line(&mut line)
        .expect("Could not read line");
    let mut splitted_line = line.trim().split_whitespace();
    let n = next_iter(&mut splitted_line);
    let s = next_iter(&mut splitted_line);
    let mut line = String::new();
    io::stdin().read_line(&mut line)
        .expect("Could not read line");
    let mut splitted_line = line.trim().split_whitespace();
    let mut all_numbers: Vec<usize> = Vec::with_capacity(n * 2);
    for _ in 0..n {
        // add numbers to vector
        let s_next_num = splitted_line.next().unwrap();
        // println!("Got string: {}", s_next_num);
        let num = s_next_num.parse::<usize>().unwrap();
        for i in 0..all_numbers.len() {
            if num + all_numbers[i] == s {
                println!("YES");
                return;
            }
        }
        let mut char_iter = s_next_num.chars();
        let can_be_flipped = char_iter.all(|x| x <= '2' || (x >= '5' && x <= '6') || x >= '8');
        if can_be_flipped {
            let flipped = s_next_num.chars().rev()
                .map(|x| flip_dictionary[&x])
                .collect::<String>();
            // println!("before parse: {}", flipped);
            let flipped_num = flipped.parse::<usize>().unwrap();
            // println!("Flipped num: {}", flipped_num);
            for i in 0..all_numbers.len() {
                if flipped_num + all_numbers[i] == s {
                    println!("YES");
                    return;
                }
            }
            all_numbers.push(flipped_num);
        }
        all_numbers.push(num);
    }
    println!("NO");
}