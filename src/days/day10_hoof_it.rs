use std::fmt;

use crate::utils::get_utility::{end_day, new_position_in_direction, start_day, Cord, Direction};

pub(crate) fn day10_hoof_it() {
    println!("Running day10 Disk Fragmenter");

    let (content, timer) = start_day("./src/inputs/day10.txt");

    let node_2d_array = content_to_node_2d_array(&content);

    let result = walk_node_2d_array(&node_2d_array);

    // print_2d_node_array(&node_2d_array);

    end_day(&result, &timer);
}

// Attempt BFS
fn walk_node_2d_array(node_2d_array: &Vec<Vec<Node>>) -> usize {
    let mut result = 0;

    for (x, node_array) in node_2d_array.iter().enumerate() {
        for (y, node) in node_array.iter().enumerate() {
            if node.hight == 0 {
                let mut working_array = node_2d_array.clone();
                result += find_nines(
                    node_2d_array.get(x).unwrap().get(y).unwrap().clone(),
                    &mut working_array,
                );
            }
        }
    }

    result
}

fn find_nines(start: Node, node_2d_array: &mut Vec<Vec<Node>>) -> usize {
    let mut result = 0;

    let mut queue = vec![start];

    while let Some(working_node) = queue.pop() {
        if working_node.hight == 9 {
            result += 1;
            continue;
        }

        for edge in get_edge(working_node, &node_2d_array) {
            let node_pointer = node_2d_array
                .get_mut(edge.x)
                .unwrap()
                .get_mut(edge.y)
                .unwrap();

            node_pointer.visited = true;

            queue.push(node_pointer.clone());
        }
    }

    result
}

const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

fn get_edge(start_node: Node, node_2d_array: &Vec<Vec<Node>>) -> Vec<Cord> {
    let mut un_explored_nodes = vec![];

    for direction in DIRECTIONS {
        if let Some(new_cord) = new_position_in_direction(&start_node.cord, &direction) {
            if let Some(row) = node_2d_array.get(new_cord.x) {
                if let Some(new_node) = row.get(new_cord.y) {
                    if new_node.hight == start_node.hight + 1 && !new_node.visited {
                        un_explored_nodes.push(new_node.cord);
                    }
                }
            }
        }
    }

    un_explored_nodes
}

#[allow(dead_code)]
fn print_2d_node_array(node_2d_array: &Vec<Vec<Node>>) {
    println!();
    for node_array in node_2d_array {
        for node in node_array {
            print!("{}", node.to_string());
        }
        println!();
    }
}

fn content_to_node_2d_array(input: &str) -> Vec<Vec<Node>> {
    let mut output = vec![];

    let mut working_vec: Vec<Node> = vec![];

    for char in input.chars() {
        if char == '\n' {
            output.push(working_vec.clone());
            working_vec.clear();
        } else {
            working_vec.push(Node {
                hight: char.to_string().parse().expect("Could not parse int"),
                cord: Cord { x: 0, y: 0 },
                visited: false,
            });
        }
    }

    output.push(working_vec);

    for (x, node_array) in output.clone().iter().enumerate() {
        for (y, _) in node_array.iter().enumerate() {
            output.get_mut(x).unwrap().get_mut(y).unwrap().cord = Cord { x, y }
        }
    }

    output
}

#[derive(Clone, Debug)]
struct Node {
    hight: usize,
    cord: Cord,
    visited: bool,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.hight)
    }
}

pub(crate) fn day10_2_hoof_it() {
    println!("Running day10.2 Disk Fragmenter");

    let (content, timer) = start_day("./src/inputs/day10_test.txt");
}
