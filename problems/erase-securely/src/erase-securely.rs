use std::io;

fn main() {
    let mut line = String::new();
    let mut bin_line_1 = String::new();
    let mut bin_line_2 = String::new();
    io::stdin().read_line(&mut line)
        .expect("could not read line");
    io::stdin().read_line(&mut bin_line_1)
        .expect("could not read line");
    io::stdin().read_line(&mut bin_line_2)
        .expect("could not read line");
    let line = line.trim();
    let bin_line_1 = bin_line_1.trim();
    let bin_line_2 = bin_line_2.trim();
    let times = line.parse::<u8>().unwrap();
    //print!("{}", times);
    
    let correct_msg = "Deletion succeeded";
    let wrong_msg = "Deletion failed";
    
    if times % 2 == 0 {
        if bin_line_1 == bin_line_2 {
            print!("{}", correct_msg);
        } else {
            print!("{}", wrong_msg);
        }
    } else {
        let l1_chars = bin_line_1.chars();
        let mut l2_chars = bin_line_2.chars();
        for bin in l1_chars {
            if l2_chars.next().unwrap() == bin {
                print!("{}", wrong_msg);
                return;
            }
        }
        print!("{}", correct_msg);
    }
}
