use std::io;

fn main() -> io::Result<()> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    let arr : [i8; 6] = [1, 1, 2, 2, 2, 8];
    let mut i = 0;
    for num in line.split_whitespace() {
        print!("{}", arr[i] - (num.parse::<i8>().unwrap()));
        i+=1;
        if i != arr.len() {
            print!(" ");
        }
    }
    Ok(())
}
