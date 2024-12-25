use std::collections::HashSet;

use crate::utils::get_utility::{content_to_2d_array, end_day, print_2d_array, start_day, Cords};

pub(crate) fn day8_resonant_collinearity() {
    println!("Running day8 Resonant Collinearity");

    let (content, timer) = start_day("./src/inputs/day8.txt");

    let mut current_2d_array = content_to_2d_array(&content, false);

    let mut anti_node_locations: HashSet<Cords> = HashSet::new();

    for (x, points) in current_2d_array.clone().iter().enumerate() {
        for (y, possible_node) in points.iter().enumerate() {
            if *possible_node != '.' {
                draw_all_possible_anti_nodes(
                    &mut current_2d_array,
                    &NodeValue {
                        value: possible_node.clone(),
                        cords: Cords { x, y },
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
    current_2d_array: &mut Vec<Vec<char>>,
    node_value: &NodeValue,
    anti_node_locations: &mut HashSet<Cords>,
) {
    for (x, layer) in current_2d_array.clone().iter().enumerate() {
        for (y, node) in layer.iter().enumerate() {
            if *node == node_value.value && (x != node_value.cords.x || y != node_value.cords.y) {
                if let Some(node) = antinode_cords(&node_value.cords, &Cords { x, y }) {
                    if let Some(row) = current_2d_array.get(node.x) {
                        if let Some(_) = row.get(node.y) {
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
    cords: Cords,
}

fn antinode_cords(node1: &Cords, node2: &Cords) -> Option<Cords> {
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
        return Some(Cords { x, y });
    }

    None
}

pub(crate) fn day8_2_resonant_collinearity() {
    println!("Running day8.2 Resonant Collinearity");

    let (content, timer) = start_day("./src/inputs/day8.txt");

    let mut current_2d_array = content_to_2d_array(&content, false);

    let mut anti_node_locations: HashSet<Cords> = HashSet::new();

    for (x, points) in current_2d_array.clone().iter().enumerate() {
        for (y, possible_node) in points.iter().enumerate() {
            if *possible_node != '.' {
                anti_node_locations.insert(Cords { x, y });
                solve_all_possible_anti_nodes(
                    &mut current_2d_array,
                    &NodeValue {
                        value: possible_node.clone(),
                        cords: Cords { x, y },
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
    anti_nodes_at_towers: &mut HashSet<Cords>,
) {
    for (x, layer) in current_2d_array.clone().iter().enumerate() {
        for (y, node) in layer.iter().enumerate() {
            if *node == node_value.value && (x != node_value.cords.x || y != node_value.cords.y) {
                solve_antinode(
                    current_2d_array,
                    &node_value.cords,
                    &Cords { x, y },
                    anti_nodes_at_towers,
                );
            }
        }
    }
}

fn solve_antinode(
    current_2d_array: &mut Vec<Vec<char>>,
    node1: &Cords,
    node2: &Cords,
    anti_node_locations: &mut HashSet<Cords>,
) {
    if let Some(node) = antinode_cords(node1, node2) {
        if let Some(row) = current_2d_array.get(node.x) {
            if let Some(_) = row.get(node.y) {
                anti_node_locations.insert(node);
                return solve_antinode(current_2d_array, &node, node1, anti_node_locations);
            }
        }
    }
}

#[allow(dead_code)]
fn draw_print_anti_nodes(anti_nodes: HashSet<Cords>, mut current_2d_array: Vec<Vec<char>>) {
    for (x, row) in current_2d_array.clone().iter().enumerate() {
        for (y, _) in row.iter().enumerate() {
            if anti_nodes.contains(&Cords { x, y }) {
                *current_2d_array.get_mut(x).unwrap().get_mut(y).unwrap() = '#';
            }
        }
    }

    print_2d_array(&current_2d_array);
}
