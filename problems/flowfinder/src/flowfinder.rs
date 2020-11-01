use std::io;

struct Node {
    parent: &Node,
    flow: usize,
    children: Vec<&Node>,
}

fn parse_next(iter: &mut std::str::SplitWhitespace) -> usize {
    return iter.next().unwrap().parse::<usize>().unwrap();
}

fn recursive_traverse(node: &mut Node) {
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
    let root = Node {
        parent: null,
        flow: parse_next(&mut splitted_line_2),
    };
    let mut nodes: Vec<Node> = Vec::with_capacity(n);
    let mut nodes_missing: Vec<usize> = Vec::with_capacity(n);
    nodes.push(root);
    // setup all other nodes
    for i in 1..n {
        let parent_index = parse_next(&mut splitted_line_1);
        let parent = &nodes[parent_index];
        let flow = parse_next(&mut splitted_line_2);
        if flow <= 0 {
            nodes_missing.push(i);
        }
        nodes.push(Node {
            parent: parent,
            flow: flow,
            children: Vec::with_capacity(2),
        });
        parent.children.push(&nodes.last().unwrap());
    }

    for i in nodes_missing {
        let node = nodes[i];
        let has_value = node.flow > 0;
        if has_value {
            continue;
        }
        let is_root = node.parent == null;
        if !is_root && node.parent.flow > 0 {
            // check parent
            let other_flow = node.parent.children.iter()
                .map(|c| c.flow)
                .sum();
            let remaining = node.parent.flow - other_flow;
            node.flow = remaining;
            continue;
        }
    }
}