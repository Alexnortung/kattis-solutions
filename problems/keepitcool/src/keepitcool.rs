use std::io;
use std::cmp;

struct Slot {
    index: i32,
    initial: i32,
    new_bottles: i32,
}

fn parse_next(iter: &mut std::str::SplitWhitespace) -> i32 {
    return iter.next().unwrap().parse::<i32>().unwrap();
}

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line)
        .expect("Could not read line");
    let mut splitted_line = line.split_whitespace();
    let mut new_bottles = parse_next(&mut splitted_line);
    let students = parse_next(&mut splitted_line);
    let slots = parse_next(&mut splitted_line);
    let slot_capacity = parse_next(&mut splitted_line);
    let mut line = String::new();
    io::stdin().read_line(&mut line)
        .expect("Could not read line");
    let mut splitted_line = line.split_whitespace();
    let mut slots_vector: Vec<Slot> = Vec::with_capacity(slots as usize);
    for i in 0..slots {
        let bottles_in_slot = parse_next(&mut splitted_line);
        let slot = Slot {
            index: i,
            initial: bottles_in_slot,
            new_bottles: 0,
        };
        match slots_vector.binary_search_by_key(&bottles_in_slot, |c| c.initial) {
            Ok(pos) => {
                slots_vector.insert(pos, slot)
            },
            Err(pos) => {
                slots_vector.insert(pos, slot)
            },
        }
    }
    // we should now have a sorted array of slots
    let mut last_fill_index = slots;
    for i in 0..slots_vector.len() {
        let remaining_capacity = slot_capacity - slots_vector[i].initial;
        if remaining_capacity <= 0 {
            break;
        }
        let fill_amount = cmp::min(remaining_capacity, new_bottles);
        // println!("remaining capacity: {}, new_bottles: {}, fill_amount: {}", remaining_capacity, new_bottles, fill_amount);
        slots_vector[i].new_bottles = fill_amount;
        new_bottles -= fill_amount;
        if new_bottles <= 0 {
            last_fill_index = i as i32;
            break;
        }
    }
    let mut cold_bottles = 0;
    for i in (last_fill_index as usize + 1)..slots_vector.len() {
        // println!("{}", slots_vector[i].initial);
        cold_bottles += slots_vector[i].initial;
    }
    if cold_bottles < students {
        println!("impossible");
        return;
    }
    slots_vector.sort_by_key(|c| c.index);
    for i in 0..(slots_vector.len() - 1) {
        print!("{} ", slots_vector[i].new_bottles);
    }
    println!("{}", slots_vector.last().unwrap().new_bottles);
}