use std::io;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::VecDeque;

struct Teque<T> {
    start_vec: VecDeque<T>,
    end_vec: VecDeque<T>,
}

impl<T> Teque<T> {
    fn new(capacity : usize) -> Teque<T> {
        Teque {
            start_vec: VecDeque::with_capacity(capacity),
            end_vec: VecDeque::with_capacity(capacity),
        }
    }
    fn push_front(&mut self, value: T) {
        self.start_vec.push_front(value);
    }
    fn push_back(&mut self, value: T) {
        self.end_vec.push_front(value);
    }
    fn push_middle(&mut self, value: T) {
        // balance the Vectors
        let total_size = self.start_vec.len() + self.end_vec.len();
        let balance : isize = ((total_size as isize) + 1) / 2; // start_vec needs to become this
        let difference : isize = (self.start_vec.len() as isize) - balance; // positive means - move difference amount of elements to end_vec
        for _ in 0..difference {
            let move_value = self.start_vec.pop_back().unwrap();
            self.end_vec.push_back(move_value);
        }
        // move from end_vec to start_vec
        for _ in difference..0 {
            let move_value = self.end_vec.pop_back().unwrap();
            self.start_vec.push_back(move_value);
        }

        if total_size % 2 == 0 { // if it is an equal number we can just as well make it balanced
            self.start_vec.push_back(value);
        } else {
            self.end_vec.push_back(value);
        }
    }

    fn get(&self, index: usize) -> &T {
        if index < self.start_vec.len() {
            return self.start_vec.get(index).unwrap();
        }
        let end_vec_index = (self.end_vec.len() - 1) - (index - self.start_vec.len());
        return self.end_vec.get(end_vec_index).unwrap();
    }
}

fn main() {
    let mut line = String::new();
    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin);
    let _len = reader.read_line(&mut line);
    let num_lines = line.trim().parse::<usize>().unwrap();
    let mut teque = Teque::<isize>::new(num_lines);
    //let mut lines_iter = reader.lines();
    for _ in 0..num_lines {
        //match lines_iter.next() {
        //    Some(Err(_)) => break,
        //    Some(Ok(line_unwrapped)) => {
        //        line = line_unwrapped;
        //    },
        //    None => break,
        //}
        line = String::new();
        let _len = reader.read_line(&mut line);
        let mut splitted_line = line.trim().split_whitespace();
        let method = splitted_line.next().unwrap();
        let num_string = splitted_line.next().unwrap();
        let value = num_string.parse::<isize>().unwrap();
        match method {
            "push_back" => teque.push_back(value),
            "push_front" => teque.push_front(value),
            "push_middle" => teque.push_middle(value),
            "get" => println!("{}", teque.get(value as usize)),
            _ => (),
        }
    }
}
