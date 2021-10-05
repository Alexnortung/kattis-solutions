use std::io;
use std::collections::VecDeque;

struct Teque {
    startVec: VecDeque<usize>,
    startVecLen: usize,
    endVec: VecDeque<usize>,
    endVecLen: usize,
}

impl Teque {
    fn new(capacity : usize) -> Teque {
        Teque {
            startVec: VecDeque::with_capacity(capacity),
            startVecLen: 0,
            endVec: VecDeque::with_capacity(capacity),
            endVecLen: 0,
        }
    }
    fn push_front(&mut self, value: usize) {
        self.startVec.push_front(value);
        self.startVecLen += 1;
    }
    fn push_back(&mut self, value: usize) {
        self.endVec.push_front(value);
        self.endVecLen += 1;
    }
    fn push_middle(&mut self, value: usize) {
        // balance the Vectors
        let total_size = self.startVecLen + self.endVecLen;
        let balance = (total_size + 1) / 2; // startVecLen needs to become this
        let difference = self.startVecLen - balance; // positive means - move difference amount of elements to endVec
        for _ in 0..difference {
            self.startVecLen -= 1;
            let moveValue = self.startVec[self.startVecLen];
            if self.endVecLen == self.endVec.len() {
                self.endVec.push_back(moveValue);
            } else {
                self.endVec[self.endVecLen] = moveValue;
            }
            self.endVecLen += 1;
        }
        // move from endVec to startVec
        for _ in difference..0 {
            self.endVecLen -= 1;
            let moveValue = self.endVec[self.endVecLen];
            if self.startVecLen == self.startVec.len() {
                self.startVec.push_back(moveValue);
            } else {
                self.startVec[self.startVecLen] = moveValue;
            }
            self.startVecLen += 1;
        }

        if total_size % 2 == 0 { // if it is an equal number we can just as well make it balanced
            if self.startVecLen == self.startVec.len() {
                self.startVec.push_back(value);
            } else {
                self.startVec[self.startVecLen] = value;
            }
            self.startVecLen += 1;
        } else {
            if self.endVecLen == self.endVec.len() {
                self.endVec.push_back(value);
            } else {
                self.endVec[self.endVecLen] = value;
            }
            self.endVecLen += 1;
        }
    }

    fn get(&self, index: usize) -> usize {
        if index < self.startVecLen {
            return self.startVec[index];
        }
        let endVecIndex = (self.endVecLen - 1) - (index - self.startVecLen);
        return self.endVec[endVecIndex];
    }
}

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line)
        .expect("Could not read line");
    let num_lines = line.trim().parse::<usize>().unwrap();
    let mut teque = Teque::new(num_lines);
    for _ in 0..num_lines {
        let mut line = String::new();
        io::stdin().read_line(&mut line)
            .expect("Could not read line");
        let mut splitted_line = line.trim().split_whitespace();
        let method = splitted_line.next().unwrap();
        let num_string = splitted_line.next().unwrap();
        let value = num_string.parse::<usize>().unwrap();
        match method {
            "push_back" => teque.push_back(value),
            "push_front" => teque.push_front(value),
            "push_middle" => teque.push_middle(value),
            "get" => println!("{}", teque.get(value)),
            _ => (),
        }
    }
}
