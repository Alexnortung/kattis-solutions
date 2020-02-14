use std::io;

fn is_valid(num: u32) -> bool {
    if num < 123456 || num > 987654 {
        return false;
    }
    let input_num = num;
    let mut num = num;
    let mut num_arr = [false; 10];
    while num != 0 {
        let digit = num % 10;
        if digit == 0 {
            return false;
        }
        if num_arr[digit as usize] || input_num % digit != 0 {
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
    let c_val = start + 1;
    for i in c_val..(end+1) {
        if is_valid(i) {
            combs += 1;
        }
    }
    
    println!("{}", combs);
    //println!("{} {}", start, end);

    Ok(())
}
