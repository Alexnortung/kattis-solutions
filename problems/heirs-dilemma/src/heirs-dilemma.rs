use std::io;

fn num_vec_to_num(num_vec: Vec<u32>) -> u32 {
    let mut num = 0u32;
    for i in 0..num_vec.len() {
        let digit = num_vec[num_vec.len() - 1 - i];
        num += 10u32.pow(i as u32) * digit;
    }
    num
}

fn num_to_num_vec(num: u32) -> Vec<u32> {
    let mut num_vec = Vec::with_capacity(6);
    let mut temp_num = num;
    while temp_num != 0 {
        let digit = temp_num % 10;
        num_vec.push(digit);
        temp_num = temp_num / 10;
    }
    num_vec.iter().rev().cloned().collect()
}

fn next_valid(num: u32) -> Option<u32> {
    let next_num = num + 1;
    if is_valid(next_num) {
        return Some(next_num);
    }
    let num_vec = num_to_num_vec(next_num);
    let mut possible_num = Vec::with_capacity(9);
    for i in 0..10 {
        if !num_vec.contains(&i) {
            possible_num.push(i);
        }
    }
    let mut final_num_vec = Vec::with_capacity(6);
    for i in (0..num_vec.len()).rev() {
        let digit = num_vec[i];
        //println!("checking digit {}", digit);
        if final_num_vec.contains(&digit) {
            match possible_num.iter().position(|&x| x > digit ) {
                Some(pos) => {
                    let new_digit = possible_num[pos];
                    possible_num.remove(pos);
                    final_num_vec.push(new_digit);
                    // find out how many numbers are missing from the final number
                    // then add the lowest numbers from possible_num to final num
                    let missing_nums = 6 - final_num_vec.len();
                    for i in 0..missing_nums {
                        final_num_vec.push(possible_num[i]);
                    }
                },
                None => {
                    // remove all digits from the end of final_num_vec until digit is reached (also removing that)
                    // if final_num_vec is empty we are finished, because that means the first digit was 9 - NOT TRUE
                    // then create a new possible_num vec where it ignores the numbers in final_num_vec
                    while final_num_vec[final_num_vec.len() - 1] != digit {
                        final_num_vec.remove(final_num_vec.len() - 1);
                    }
                    final_num_vec.remove(final_num_vec.len() - 1); // removing the first occurrence of digit as well
                    let numbers = final_num_vec.len();
                    if numbers == 0 {
                        // return none
                        return None;
                    }
                    let temp_num = num_vec_to_num(final_num_vec) + 1;
                    let mut final_num_vec = num_to_num_vec(temp_num);
                    if final_num_vec.len() != numbers {
                        return None;
                    }
                    let mut possible_num = Vec::with_capacity(9);
                    for i in 0..10 {
                        if !final_num_vec.contains(&i) {
                            possible_num.push(i);
                        }
                    }
                    for i in 0..(6 - numbers) {
                        final_num_vec.push(possible_num[i]);
                    }
                    return Some(num_vec_to_num(final_num_vec));
                },
            }
            break;
        }
        final_num_vec.push(digit);
    }
    Some(num_vec_to_num(final_num_vec))
}

fn is_valid(num: u32) -> bool {
    //if num < 123456 || num > 987654 {
    //    return false;
    //}
    let mut num = num;
    let mut num_arr = [false; 10];
    while num != 0 {
        let digit = num % 10;
        if num_arr[digit as usize] {
            return false;
        }
        num_arr[digit as usize] = true;
        num = num / 10;
    }
    true
}

fn main() -> io::Result<()> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    let mut line_iter = line.trim().split(' ');
    let start: u32 = line_iter.next().unwrap().parse()
        .expect("could not parse");
    let end: u32 = line_iter.next().unwrap().parse()
        .expect("could not parse");
    
    let mut combs = 0;
    if is_valid(start) {
        combs += 1;
    }
    let mut value = next_valid(start);
    loop {
        match value {
            Some(num) => {
                if num > end {
                    break;
                }
                value = next_valid(num);
                println!("{} is valid", num);
                combs += 1;
            },
            None => break,
        }
    }
    
    println!("{}", combs);
    //println!("{} {}", start, end);

    Ok(())
}
