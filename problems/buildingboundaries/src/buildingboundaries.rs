
use std::io;
use std::cmp;

fn parse_next(iter: &mut std::str::SplitWhitespace) -> usize {
    iter.next().unwrap().parse::<usize>().unwrap()
}

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line)
        .expect("Could not read line");
    let num_lines = line.trim().parse::<usize>().unwrap();
    for _ in 0..num_lines {
        line = String::new();
        io::stdin().read_line(&mut line)
            .expect("Could not read line");
        let mut splitted_line = line.trim().split_whitespace();
        let a1 = parse_next(&mut splitted_line);
        let b1 = parse_next(&mut splitted_line);
        let a2 = parse_next(&mut splitted_line);
        let b2 = parse_next(&mut splitted_line);
        let a3 = parse_next(&mut splitted_line);
        let b3 = parse_next(&mut splitted_line);
        let mut vec_vals = vec![a1, b1, a2, b2, a3, b3];
        // println!("{:?}", vec_vals);
        let mut max_index = 0;
        let mut max_value = 0;
        for i in 0..6 {
            if vec_vals[i] > max_value {
                // println!("vec_vals[i] {} > max {}, i: {}", vec_vals[i], max_value, i);
                max_value = vec_vals[i];
                max_index = i;
            }
        }
        let rect_index = (max_index / 2) * 2;
        // println!("rect index: {}", rect_index);
        let main_x = vec_vals.remove(rect_index);
        let main_y = vec_vals.remove(rect_index);
        let x_2 = vec_vals[0];
        let y_2 = vec_vals[1];
        let x_3 = vec_vals[2];
        let y_3 = vec_vals[3];
        let mut min_area = std::usize::MAX;
        for i in 0..4 {
            let (x_2, y_2) = if i % 2 == 0 { (x_2, y_2) } else { (y_2, x_2) };
            // let start_x_2 = i < 2 ? main_x : 0;
            // let start_y_2 = i < 2 ? 0 : main_y;
            if i < 2 {
                // rect 2 is on bottom
                let start_x_2 = main_x;
                let start_y_2 = 0;
                for j in 0..6 {
                    let (x_3, y_3) = if j % 2 == 0 { (x_3, y_3) } else { (y_3, x_3) };
                    let start_x_3;
                    let start_y_3;
                    if j < 2 {
                        start_x_3 = 0;
                        start_y_3 = main_y;
                    } else if j < 4 {
                        start_x_3 = main_x;
                        start_y_3 = y_2;
                    } else {
                        start_x_3 = start_x_2 + x_2;
                        start_y_3 = 0;
                    }
                    let area = calculate_area(main_x, main_y, start_x_2, start_y_2, x_2, y_2, start_x_3, start_y_3, x_3, y_3);
                    min_area = cmp::min(min_area, area);
                }
            } else {
                // rect 2 is on top of main
                let start_x_2 = 0;
                let start_y_2 = main_y;
                for j in 0..6 {
                    let (x_3, y_3) = if j % 2 == 0 { (x_3, y_3) } else { (y_3, x_3) };
                    let start_x_3;
                    let start_y_3;
                    if j < 2 {
                        start_x_3 = main_x;
                        start_y_3 = 0;
                    } else if j < 4 {
                        start_x_3 = x_2;
                        start_y_3 = main_y;
                    } else {
                        start_x_3 = 0;
                        start_y_3 = start_y_2 + y_2;
                    }
                    let area = calculate_area(main_x, main_y, start_x_2, start_y_2, x_2, y_2, start_x_3, start_y_3, x_3, y_3);
                    min_area = cmp::min(min_area, area);
                }
            }
        }
        println!("{}", min_area);
    }
}

fn calculate_area(mx: usize, my: usize, p2x: usize, p2y: usize, x2: usize, y2: usize, p3x: usize, p3y: usize, x3: usize, y3: usize) -> usize {
    // right
    let mut right = mx;
    let mut temp = p2x + x2;
    if temp > right {
        right = temp;
    }
    temp = p3x + x3;
    if temp > right {
        right = temp;
    }
    // top
    let mut top = my;
    temp = p2y + y2;
    if temp > top {
        top = temp;
    }
    temp = p3y + y3;
    if temp > top {
        top = temp;
    }

    return right * top;
}