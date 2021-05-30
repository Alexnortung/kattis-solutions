use std::io;
//use std::vec;

fn main() {
    let mut vec = Vec::with_capacity(10);
    for _ in 0..10 {
        let mut line = String::new();
        io::stdin().read_line(&mut line)
            .expect("Could not read line");
        let num = line.trim().parse::<usize>().unwrap();
        let insert_num = num % 42;
        match vec.iter().any(|&x| &x == &insert_num) {
            true => (),
            false => vec.push(insert_num),
        }
    }
    println!("{}", vec.len());
}
