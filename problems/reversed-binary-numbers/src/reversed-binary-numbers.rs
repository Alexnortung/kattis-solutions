// Problem at:      https://open.kattis.com/problems/reversebinary
// Submission at:   https://open.kattis.com/submissions/5206616
use std::io;
use std::convert::TryInto;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line);
    let number : u32 = line.trim().parse()
        .expect("input isn't a number");
    let mut checknumber = number;
    let mut reversed = 0u32;
    let mut v = Vec::with_capacity(31);
    while checknumber != 0 {
        let bit = match checknumber % 2 {
            1 => true,
            0 => false,
            _ => true,
        };
        v.push(bit);
        checknumber = checknumber >> 1;
    }

    let len = v.len();
    for i in 0..len {
        if v[i] {
            reversed += 2_u32.pow((len - i - 1).try_into().unwrap());
        }

    }
    println!("{}", reversed);
}
