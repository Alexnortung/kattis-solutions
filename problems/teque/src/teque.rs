use std::io;
use std::collections::VecDeque;

struct Teque<T> {
    startVec: VecDeque<T>,
    endVec: VecDeque<T>,
}

impl<T> Teque<T> {
    fn new(capacity : usize) -> Teque<T> {
        Teque {
            startVec: VecDeque::with_capacity(capacity),
            endVec: VecDeque::with_capacity(capacity),
        }
    }
    fn push_front(&mut self, value: T) {
        self.startVec.push_front(value);
    }
    fn push_back(&mut self, value: T) {
        self.endVec.push_front(value);
    }
    fn push_middle(&mut self, value: T) {
        // balance the Vectors
        let total_size = self.startVec.len() + self.endVec.len();
        let balance : isize = ((total_size as isize) + 1) / 2; // startVecLen needs to become this
        let difference : isize = (self.startVec.len() as isize) - balance; // positive means - move difference amount of elements to endVec
        for _ in 0..difference {
            let moveValue = self.startVec.pop_back().unwrap();
            self.endVec.push_back(moveValue);
        }
        // move from endVec to startVec
        for _ in difference..0 {
            let moveValue = self.endVec.pop_back().unwrap();
            self.startVec.push_back(moveValue);
        }

        if total_size % 2 == 0 { // if it is an equal number we can just as well make it balanced
            self.startVec.push_back(value);
        } else {
            self.endVec.push_back(value);
        }
    }

    fn get(&self, index: usize) -> &T {
        if index < self.startVec.len() {
            return self.startVec.get(index).unwrap();
        }
        let endVecIndex = (self.endVec.len() - 1) - (index - self.startVec.len());
        return self.endVec.get(endVecIndex).unwrap();
    }
}

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line)
        .expect("Could not read line");
    let num_lines = line.trim().parse::<usize>().unwrap();
    let mut teque = Teque::<isize>::new(num_lines);
    for _ in 0..num_lines {
        let mut line = String::new();
        io::stdin().read_line(&mut line)
            .expect("Could not read line");
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
