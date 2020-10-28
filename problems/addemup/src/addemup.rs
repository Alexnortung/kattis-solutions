use std::io;

fn next_iter(iter: &mut std::str::SplitWhitespace) -> usize {
    return iter.next().unwrap().parse::<usize>().unwrap();
}

fn main() {
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
        let num = s_next_num.parse::<usize>().unwrap();
        let mut char_iter = s_next_num.chars();
        let can_be_flipped = char_iter.all(|x| x <= '2' || (x >= '5' && x <= '6') || x >= '8');
        if can_be_flipped {
            let flipped = s_next_num.chars().rev()
                .map(|x| match x {
                    '6' => '9',
                    '9' => '6',
                    _ => x,
                })
                .collect::<String>();
            let flipped_num = flipped.parse::<usize>().unwrap();
            for i in 0..all_numbers.len() {
                if flipped_num + all_numbers[i] == s || num + all_numbers[i] == s {
                    println!("YES");
                    return;
                }
            }
            all_numbers.push(flipped_num);
        } else {
            for i in 0..all_numbers.len() {
                if num + all_numbers[i] == s {
                    println!("YES");
                    return;
                }
            }
        }

        all_numbers.push(num);
    }
    println!("NO");
}