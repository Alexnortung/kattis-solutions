use std::io;
use std::ptr;

struct Node {
    flow: usize,
    children: Vec<Node>,
    solved: bool,
}

impl Node {
    fn sum_children(&self) -> usize {
        let mut accumulator = 0;
        for child in &self.children {
            accumulator += child.flow;
        }
        accumulator
    }
}

fn parse_next(iter: &mut std::str::SplitWhitespace) -> usize {
    return iter.next().unwrap().parse::<usize>().unwrap();
}

fn solve_tree(node: &mut Node) -> bool {
    if node.solved {
        return true;
    }
    // does this node have a value?
    let mut has_value = node.flow != 0;
    let is_leaf = node.children.len() == 0;
    if is_leaf {
        node.solved = has_value;
        return has_value;
    }
    let children_with_values = node.children.iter().fold(0, |acc, child| acc + (child.flow != 0) as usize);
    let diff_children = node.children.len() - children_with_values;
    match (has_value, diff_children) {
        (true, 1) => {
            let sum = node.sum_children();
            let mut zero_node = node.children.iter_mut().find(|child| child.flow == 0).unwrap();
            zero_node.flow = node.flow - sum;
        },
        (false, 0) => {
            node.flow = node.sum_children();
            has_value = true;
        },
        _ => (),
    }

    let mut children_results = Vec::with_capacity(2);
    for mut child in &mut node.children {
        let solved = if child.solved { true } else { solve_tree(&mut child) };
        children_results.push(solved);
    }
    let num_children_solved = children_results.iter().fold(0, |acc, solved| acc + (*solved as usize));
    let diff_children_solved = node.children.len() - num_children_solved;
    match (diff_children_solved, has_value) {
        (0, false) => {
            node.flow = node.sum_children();
            node.solved = true;
            return true;
        },
        (0, true) => {
            node.solved = true;
            return true;
        },
        (1, true) => {
            let sum = node.sum_children();
            if let Some(mut zero_node) = node.children.iter_mut().find(|child| child.flow == 0) {
                zero_node.flow = node.flow - sum;
                let solved = solve_tree(&mut zero_node);
                node.solved = solved;
                return solved;
            }
            node.solved = false;
            return false;
        },
        _ => return false,
    }
}

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line)
        .expect(".");
    let n = line.trim().parse::<usize>().unwrap();
    let mut line = String::new();
    io::stdin().read_line(&mut line)
        .expect(".");
    let mut splitted_line_1 = line.trim().split_whitespace();
    let mut line = String::new();
    io::stdin().read_line(&mut line)
        .expect(".");
    let mut splitted_line_2 = line.trim().split_whitespace();
    let mut root = Node {
        flow: parse_next(&mut splitted_line_2),
        children: Vec::with_capacity(2),
        solved: false,
    };
    //let mut nodes: Vec<*mut isize> = Vec::with_capacity(n);
    //let mut nodes: Vec<&mut Node> = Vec::with_capacity(n);
    //let mut nodes_missing: Vec<usize> = Vec::with_capacity(n);
    let mut nodes: Vec<*mut Node> = Vec::with_capacity(n);
    nodes.push(ptr::addr_of_mut!(root));
    // setup all other nodes
    for _ in 1..n {
        let _parent_index = parse_next(&mut splitted_line_1) - 1;
        let parent = nodes[_parent_index];
        let flow = parse_next(&mut splitted_line_2);
        let node = Node {
            flow: flow,
            children: Vec::with_capacity(2),
            solved: false,
        };
        unsafe {
            (*parent).children.push(node);
            let new_index = (*parent).children.len() - 1;
            nodes.push(ptr::addr_of_mut!((*parent).children[new_index]));
        }
    }

    if !solve_tree(&mut root) {
        println!("impossible");
        return ();
    }

    unsafe {
        for node in nodes {
            let node = &*node;
            println!("{}", node.flow);
        }
    }
}
