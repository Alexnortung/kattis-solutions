// problem at:      https://open.kattis.com/problems/addingwords
// Submitted to:    https://open.kattis.com/submissions/5397147
use std::collections::HashMap;
use std::io;

fn main() {
    let mut vars : HashMap<String, i16> = HashMap::new();
    //let mut values_to_vars : HashMap<i16, String> = HashMap::new();

    loop {
        let mut line = String::new();
        let read_bytes = io::stdin().read_line(&mut line)
            .expect("Could not read line");
        if read_bytes == 0 {
            // reached EOF (end of file)
            break;
        }
        let mut splitted_line = line.trim().split_whitespace();
        match splitted_line.next() {
            Some("clear") => {
                vars.clear();
                //values_to_vars.clear();
            },
            Some("def") => {
                let name = splitted_line.next().unwrap();
                let value = splitted_line.next().unwrap()
                    .parse::<i16>().unwrap();
                if vars.contains_key(name) {
                    let name = name.to_string();
                    //let past_value = vars.get(name);
                    //values_to_vars.remove(past_value);
                    vars.remove(&name);
                }
                let name = name.to_string();
                vars.insert(name, value);
            },
            Some("calc") => {
                let mut splitted_line = splitted_line.peekable();
                let first_name = splitted_line.next().unwrap();
                let mut sum;
                if !vars.contains_key(first_name) {
                    sum = None;
                } else {
                    sum = Some(vars.get(first_name).unwrap().to_owned());
                }
                print!("{}", first_name);
                while splitted_line.peek() != Some(&"=") && splitted_line.peek() != None && sum != None {
                    let operator = splitted_line.next().unwrap();
                    let name = splitted_line.next().unwrap();
                    print!(" {} {}", operator, name);
                    if !vars.contains_key(name) {
                        sum = None;
                        break;
                    }
                    //println!("(adding {} to sum: {:?})", name, sum);
                    let c_sum = sum.unwrap().to_owned();
                    let value = vars.get(name).unwrap();
                    match operator {
                        "+" => sum = Some( c_sum + value ),
                        "-" => sum = Some( c_sum - value ),
                        _ => (),
                    }
                    //print!("new sum is {:?}", sum);
                }
                for s in splitted_line {
                    print!(" {}", s);
                }
                print!(" ");
                match sum {
                    Some(value) => {
                        // find the name and print it else print unknow
                        let mut found = false;
                        for (var, map_value) in vars.iter() {
                            if map_value.to_owned() == value {
                                println!("{}", var);
                                found = true;
                                break;
                            }
                        }
                        if !found {
                            println!("unknown");
                        }

                    },
                    None => println!("unknown"),
                }
            },
            _ => (),

            
        }
    }
}
