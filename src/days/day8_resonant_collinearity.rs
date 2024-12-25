use std::collections::HashSet;

use crate::utils::get_utility::{content_to_2d_array, end_day, print_2d_array, start_day, Cord};

pub(crate) fn day8_resonant_collinearity() {
    println!("Running day8 Resonant Collinearity");

    let (content, timer) = start_day("./src/inputs/day8.txt");

    let mut current_2d_array = content_to_2d_array(&content, false);

    let mut anti_node_locations: HashSet<Cord> = HashSet::new();

    for (x, points) in current_2d_array.clone().iter().enumerate() {
        for (y, possible_node) in points.iter().enumerate() {
            if *possible_node != '.' {
                draw_all_possible_anti_nodes(
                    &mut current_2d_array,
                    &NodeValue {
                        value: *possible_node,
                        cord: Cord { x, y },
                    },
                    &mut anti_node_locations,
                );
            }
        }
    }

    // print_2d_array(&current_2d_array);

    end_day(&anti_node_locations.len(), &timer);
}

fn draw_all_possible_anti_nodes(
    current_2d_array: &mut [Vec<char>],
    node_value: &NodeValue,
    anti_node_locations: &mut HashSet<Cord>,
) {
    for (x, layer) in current_2d_array.to_owned().iter().enumerate() {
        for (y, node) in layer.iter().enumerate() {
            if *node == node_value.value && (x != node_value.cord.x || y != node_value.cord.y) {
                if let Some(node) = antinode_cord(&node_value.cord, &Cord { x, y }) {
                    if let Some(row) = current_2d_array.get(node.x) {
                        if row.get(node.y).is_some() {
                            anti_node_locations.insert(node);
                        }
                    }
                }
            }
        }
    }
}

struct NodeValue {
    value: char,
    cord: Cord,
}

fn antinode_cord(node1: &Cord, node2: &Cord) -> Option<Cord> {
    if let (Some(x), Some(y)) = (
        node1
            .x
            .checked_mul(2)
            .and_then(|val| val.checked_sub(node2.x)),
        node1
            .y
            .checked_mul(2)
            .and_then(|val| val.checked_sub(node2.y)),
    ) {
        return Some(Cord { x, y });
    }

    None
}

pub(crate) fn day8_2_resonant_collinearity() {
    println!("Running day8.2 Resonant Collinearity");

    let (content, timer) = start_day("./src/inputs/day8.txt");

    let mut current_2d_array = content_to_2d_array(&content, false);

    let mut anti_node_locations: HashSet<Cord> = HashSet::new();

    for (x, points) in current_2d_array.clone().iter().enumerate() {
        for (y, possible_node) in points.iter().enumerate() {
            if *possible_node != '.' {
                anti_node_locations.insert(Cord { x, y });
                solve_all_possible_anti_nodes(
                    &mut current_2d_array,
                    &NodeValue {
                        value: *possible_node,
                        cord: Cord { x, y },
                    },
                    &mut anti_node_locations,
                );
            }
        }
    }

    // print_2d_array(&current_2d_array);

    // draw_print_anti_nodes(anti_node_locations.clone(), current_2d_array);

    end_day(&anti_node_locations.len(), &timer);
}

fn solve_all_possible_anti_nodes(
    current_2d_array: &mut Vec<Vec<char>>,
    node_value: &NodeValue,
    anti_nodes_at_towers: &mut HashSet<Cord>,
) {
    for (x, layer) in current_2d_array.clone().iter().enumerate() {
        for (y, node) in layer.iter().enumerate() {
            if *node == node_value.value && (x != node_value.cord.x || y != node_value.cord.y) {
                solve_antinode(
                    current_2d_array,
                    &node_value.cord,
                    &Cord { x, y },
                    anti_nodes_at_towers,
                );
            }
        }
    }
}

fn solve_antinode(
    current_2d_array: &mut Vec<Vec<char>>,
    node1: &Cord,
    node2: &Cord,
    anti_node_locations: &mut HashSet<Cord>,
) {
    if let Some(node) = antinode_cord(node1, node2) {
        if let Some(row) = current_2d_array.get(node.x) {
            if row.get(node.y).is_some() {
                anti_node_locations.insert(node);
                return solve_antinode(current_2d_array, &node, node1, anti_node_locations);
            }
        }
    }
}

#[allow(dead_code)]
fn draw_print_anti_nodes(anti_nodes: HashSet<Cord>, mut current_2d_array: Vec<Vec<char>>) {
    for (x, row) in current_2d_array.clone().iter().enumerate() {
        for (y, _) in row.iter().enumerate() {
            if anti_nodes.contains(&Cord { x, y }) {
                *current_2d_array.get_mut(x).unwrap().get_mut(y).unwrap() = '#';
            }
        }
    }

    print_2d_array(&current_2d_array);
}
