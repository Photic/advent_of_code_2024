use std::collections::HashSet;

use crate::utils::get_utility::{content_to_2d_array, end_day, print_2d_array, start_day, Cords};

pub(crate) fn day8_resonant_collinearity() {
    println!("Running day8 Resonant Collinearity");

    let (content, timer) = start_day("./src/inputs/day8.txt");

    let mut current_2d_array = content_to_2d_array(&content, false);

    let mut anti_node_locations: HashSet<Cords> = HashSet::new();

    for (x, points) in current_2d_array.clone().iter().enumerate() {
        for (y, possible_node) in points.iter().enumerate() {
            let mut found_node_value: Option<NodeValue> = None;
            if *possible_node != '.' {
                found_node_value = Some(NodeValue {
                    value: possible_node.clone(),
                    cords: Cords { x, y },
                });
            }

            if let Some(node_value) = found_node_value {
                draw_all_possible_anti_nodes(
                    &mut current_2d_array,
                    &node_value,
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
    anti_nodes_at_towers: &mut HashSet<Cords>,
) {
    for (x, layer) in current_2d_array.clone().iter().enumerate() {
        for (y, node) in layer.iter().enumerate() {
            if *node == node_value.value && (x != node_value.cords.x || y != node_value.cords.y) {
                draw_antinode(
                    current_2d_array,
                    &node_value.cords,
                    &Cords { x, y },
                    anti_nodes_at_towers,
                );
            }
        }
    }
}

struct NodeValue {
    value: char,
    cords: Cords,
}

fn draw_antinode(
    current_2d_array: &mut Vec<Vec<char>>,
    node1: &Cords,
    node2: &Cords,
    anti_node_locations: &mut HashSet<Cords>,
) {
    for node in antinode_cords(node1, node2) {
        if let Some(row) = current_2d_array.get(node.x) {
            if let Some(_) = row.get(node.y) {
                // if *anti_node == '.' {
                //     *anti_node = '#';
                // } else {

                // }

                anti_node_locations.insert(node);
            }
        }
    }
}

fn antinode_cords(node1: &Cords, node2: &Cords) -> Vec<Cords> {
    let mut cords = Vec::new();

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
        cords.push(Cords { x, y });
    }

    if let (Some(x), Some(y)) = (
        node2
            .x
            .checked_mul(2)
            .and_then(|val| val.checked_sub(node1.x)),
        node2
            .y
            .checked_mul(2)
            .and_then(|val| val.checked_sub(node1.y)),
    ) {
        cords.push(Cords { x, y });
    }

    cords
}

pub(crate) fn day8_2_resonant_collinearity() {
    println!("Running day8.2 Resonant Collinearity");

    let (content, timer) = start_day("./src/inputs/day8_test.txt");
}
