use std::io;

fn next_iter(iter: &mut std::str::SplitWhitespace) -> i32 {
    return iter.next().unwrap().parse::<i32>().unwrap();
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
    let mut all_numbers: Vec<i32> = Vec::with_capacity((n * 2) as usize);
    for _ in 0..n {
        // add numbers to vector
        let s_next_num = splitted_line.next().unwrap();
        let num = s_next_num.parse::<i32>().unwrap();
        let find_num = s - num;
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
            let flipped_num = flipped.parse::<i32>().unwrap();
            let find_flip = s - flipped_num;
            match all_numbers.binary_search(&find_flip) {
                Ok(_) => {
                    println!("YES");
                    return;
                },
                Err(_) => (),
            }
            match all_numbers.binary_search(&find_num) {
                Ok(_) => {
                    println!("YES");
                    return;
                },
                Err(_) => (),
            }
            match all_numbers.binary_search(&flipped_num) {
                Ok(_) => (),
                Err(pos) => all_numbers.insert(pos, flipped_num),
            }
        } else {
            match all_numbers.binary_search(&find_num) {
                Ok(_) => {
                    println!("YES");
                    return;
                },
                Err(_) => (),
            }
        }

        // all_numbers.push(num);
        match all_numbers.binary_search(&num) {
            Ok(_) => (),
            Err(pos) => all_numbers.insert(pos, num),
        }
    }
    println!("NO");
}