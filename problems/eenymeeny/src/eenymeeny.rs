use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line)
        .expect("Could not read line");
    let splitted_line = line.trim().split_whitespace();
    let rhyme_number = splitted_line.count();
    line = String::new();
    io::stdin().read_line(&mut line)
        .expect("Could not read line");
    let number_of_people = line.trim().parse::<usize>().unwrap();
    let mut names: Vec<String> = Vec::with_capacity(number_of_people);
    for _ in 0..number_of_people {
        let mut line = String::new();
        io::stdin().read_line(&mut line)
            .expect("Could not read line");
        names.push(line.trim().to_string());
    }

    let mut team1 = Vec::with_capacity((number_of_people / 2) + 1);
    let mut team2 = Vec::with_capacity(number_of_people / 2);
    let mut last_index = 0;
    for i in 0..names.len() {
        let next_index = (last_index + (rhyme_number - 1)) % names.len();
        last_index = next_index;
        let name = names.remove(next_index);
        if i % 2 == 0 {
            team1.push(name);
        } else {
            team2.push(name);
        }
    }
    println!("{}", team1.len());
    for name in team1 {
        println!("{}", name);
    }
    println!("{}", team2.len());
    for name in team2 {
        println!("{}", name);
    }

}