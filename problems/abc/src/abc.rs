use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line)
        .expect(".");
    let mut splitted_line = line.trim().split_whitespace();
    let n1 = splitted_line.next().unwrap().parse::<u8>().unwrap();
    let n2 = splitted_line.next().unwrap().parse::<u8>().unwrap();
    let n3 = splitted_line.next().unwrap().parse::<u8>().unwrap();
    let mut vector: Vec<u8> = vec![n1, n2, n3];
    vector.sort();
    let mut line = String::new();
    io::stdin().read_line(&mut line)
        .expect(".");
    let chars = line.trim().chars();
    for (i, c) in chars.enumerate() {
        let c = vector[c as usize - 'A' as usize];
        print!("{}", c);
        if i == 2 {
            break;
        }
        print!(" ");
    }
    println!("");
}