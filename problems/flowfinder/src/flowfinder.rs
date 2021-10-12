use std::io;

#[derive(Debug)]
struct Node {
    flow: usize,
    children: Vec<Box<Node>>,
    solved: bool,
}

impl Node {
    fn sum_children(&self) -> usize {
        let mut accumulator: usize = 0;
        for child in &self.children {
            accumulator += child.flow as usize;
        }
        accumulator
    }
}

fn parse_next(iter: &mut std::str::SplitWhitespace) -> usize {
    return iter.next().unwrap().parse::<usize>().unwrap();
}

// returns the number of zero leafs
fn solve_zero_leafs(node: &mut Node) -> (usize, bool) {
    if node.solved {
        return (0, true);
    }
    if node.children.len() == 0 {
        node.flow = 1;
        node.solved = true;
        return (1, true);
    }

    let children_sum = node.sum_children();
    // find leafs
    let mut zero_leafs = 0;
    for mut child in &mut node.children {
        let (tmp_zero_leafs, tmp_solvable) = solve_zero_leafs(&mut child);
        if !tmp_solvable {
            return (0, false);
        }
        zero_leafs += tmp_zero_leafs;
    }

    if node.flow == 0 {
        return (zero_leafs, true);
    }

    if node.flow as isize - children_sum as isize != zero_leafs as isize {
        return (zero_leafs, false);
    }
    // it is possible to solve this

    solve_tree(node);
    return (0, true);
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
            let zero_node_flow: isize = node.flow as isize - sum as isize;
            if zero_node_flow <= 0 {
                return false;
            }
            zero_node.flow = zero_node_flow as usize;
        },
        (false, 0) => {
            node.flow = node.sum_children();
            has_value = true;
        },
        _ => (),
    }

    let mut children_results = Vec::with_capacity(node.children.len());
    for mut child in &mut node.children {
        let solved = if child.solved { true } else { solve_tree(&mut child) };
        children_results.push(solved);
    }
    let num_children_solved = children_results.iter().fold(0, |acc, solved| acc + (*solved as usize));
    let diff_children_solved: usize = node.children.len() - num_children_solved;
    match (diff_children_solved, has_value) {
        (0, false) => {
            node.flow = node.sum_children();
            node.solved = true;
            return true;
        },
        (0, true) => {
            if node.sum_children() != node.flow {
                return false;
            }
            node.solved = true;
            return true;
        },
        (1, true) => {
            let sum = node.sum_children();
            let zero_node_flow: isize = node.flow as isize - sum as isize;
            if zero_node_flow <= 0 {
                return false;
            }
            if let Some(mut zero_node) = node.children.iter_mut().find(|child| child.flow == 0) {
                zero_node.flow = zero_node_flow as usize;
                let solved = solve_tree(&mut zero_node);
                node.solved = solved;
                return solved;
            }
            node.solved = false;
            return false;
        },
        //(_, true) => {
        //    let sum = node.sum_children();
        //    let zero_children_iter = node.children.iter_mut().filter(|child| child.flow == 0);
        //    let zero_children: Vec<&mut Box<Node>> = zero_children_iter.collect();
        //    //println!("{:?}", zero_children);
        //    if diff_children_solved == zero_children.len() && node.flow - sum == zero_children.len() {
        //        for mut child in zero_children {
        //            child.flow = 1;
        //            if !solve_tree(child) {
        //                return false;
        //            }
        //            child.solved = true;
        //        }
        //        node.solved = true;
        //        return true;
        //    }
        //    return false;
        //},
        _ => return false,
    }
}

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line)
        .expect(".");
    let n: usize = line.trim().parse::<usize>().unwrap();
    let mut line = String::new();
    io::stdin().read_line(&mut line)
        .expect(".");
    let mut splitted_line_1 = line.trim().split_whitespace();
    let mut line = String::new();
    io::stdin().read_line(&mut line)
        .expect(".");
    let mut splitted_line_2 = line.trim().split_whitespace();
    let root = Node {
        flow: parse_next(&mut splitted_line_2),
        children: Vec::with_capacity(2),
        solved: false,
    };
    let mut nodes: Vec<*mut Node> = Vec::with_capacity(n);
    let mut boxed_root = Box::new(root);
    let root_pointer: *mut Node = &mut *boxed_root;
    nodes.push(root_pointer);
    // setup all other nodes
    for _ in 1..n {
        let _parent_index: usize = parse_next(&mut splitted_line_1) - 1;
        let parent = nodes[_parent_index];
        let flow = parse_next(&mut splitted_line_2);
        let node = Node {
            flow: flow,
            children: Vec::with_capacity(2),
            solved: false,
        };
        let mut boxed_node = Box::new(node);
        let node_pointer: *mut Node = &mut *boxed_node;
        unsafe {
            (*parent).children.push(boxed_node);
            //let new_index = (*parent).children.len() - 1;
            //let node_pointer: *mut Node = &mut ((*parent).children[new_index]);
            nodes.push(node_pointer);
        }
    }

    if !solve_tree(&mut boxed_root) {
        let (_, solved) = solve_zero_leafs(&mut boxed_root);
        if !solved {
            println!("impossible");
            return ();
        }
    }

    unsafe {
        for node in nodes {
            let node = &*node;
            println!("{}", node.flow);
        }
    }
}
