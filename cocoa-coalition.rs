// Problem at:      https://open.kattis.com/problems/cocoacoalition
// Submitted to:    https://open.kattis.com/submissions/5207349
use std::io;
use std::cmp;

fn next_parse(line_iter: &mut std::str::Split<'_, &str>) -> i64 {
    line_iter.next().unwrap().parse().expect("could not parse to i64")
}

fn main() -> io::Result<()> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)
        .expect("could not read line");
    let line = line.trim();
    let mut line_iter = line.split(" ");
    let m = next_parse(&mut line_iter);
    let n = next_parse(&mut line_iter);
    let a = next_parse(&mut line_iter);
    let mut found = false;

    if a % m == 0 || a % n == 0 {
        println!("1");
        found = true;
        return Ok(());
    }

    if !found {
        let mut num_check = 1;
        let num_stop: i64 = (a as f64).sqrt() as i64 + 1;
        let min_side = cmp::min(m, n);
        let max_side = cmp::max(m, n);
        while num_check <= num_stop && num_check <= min_side {
            let other_side = a as f64 / num_check as f64;
            if other_side.fract() == 0.0 && other_side <= max_side as f64 {
                println!("2");
                found = true;
                break;
            }

            let new_min = min_side - num_check;
            //println!("new min: {}",new_min);
            if new_min >= 1 {
                let first_break_size = num_check * max_side;
                let next_a = a - first_break_size;
                let max_side_check = next_a as f64 / new_min as f64;
                if max_side_check.fract() == 0.0 && next_a % new_min == 0 && max_side_check <= max_side as f64 && next_a > 0 {
                    println!("2");
                    found = true;
                    break;
                }
            }

            num_check += 1;
        }
    }

    if !found {
        println!("3");
    }

    Ok(())
}
