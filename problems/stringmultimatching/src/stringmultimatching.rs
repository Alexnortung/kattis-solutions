use std::io;

fn main() {
    // loop until we cant read any more lines
    loop {
        // Read number of lines
        let mut line = String::new();
        let read_result = io::stdin().read_line(&mut line);
        // if there was an error we are finished
        match read_result {
            Ok(_) => match check_patterns(line) {
                true => (),
                false => break,
            },
            _ => break,
        }
    }
}

fn check_patterns(line: String) -> bool {
    if line.trim().is_empty() {
        return false;
    }
    //println!("got line: {}", line);
    // Parse the line to an integer for the number of patterns
    let num = line.trim().parse::<usize>().unwrap();
    // store the patterns in a vector
    let mut patterns = Vec::with_capacity(num);
    // read the num next lines

    for _ in 0..num {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("could not read line");
        // trim and insert pattern
        patterns.push(line.trim().to_string());
    }
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("could not read line");
    let string_to_match = line.trim();

    for pattern in patterns.iter() {
        let mut last_index = 0;
        let mut indicies = Vec::new();
        loop {
            let string_to_match = &string_to_match[(last_index)..];
            //println!("{}", string_to_match);
            match string_to_match.find(&pattern[..]) {
                Some(index) => {
                    indicies.push(last_index + index);
                    last_index = last_index + index + 1;
                },
                None => break,
            }
        }
        // print indicies
        let str_indicies: Vec<String> = indicies.iter().map(|&index| index.to_string()).collect();
        println!("{}", str_indicies.join(" "));
    }
    true
}
