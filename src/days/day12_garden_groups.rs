use std::fmt;

use crate::utils::get_utility::{end_day, new_position_in_direction, start_day, Cord, DIRECTIONS};

pub(crate) fn day12_garden_groups() {
    println!("Running day12 Garden Groups");

    let (content, timer) = start_day("./src/inputs/day12.txt");

    let mut garden_2d_area = content_to_garden_2d_area(&content);

    // print_garden_to_array(&garden_2d_area);

    let result = walk_the_garden(&mut garden_2d_area);

    end_day(&result, &timer);
}

fn walk_the_garden(garden_2d_area: &mut Vec<Vec<Node>>) -> usize {
    let mut result = 0;

    for node_array in garden_2d_area.clone() {
        for node in node_array {
            let node_pointer = &garden_2d_area[node.cord.x][node.cord.y];

            if !node_pointer.visited.clone() && node_pointer.plant != '*' {
                let garden_area = find_similiar(node.clone(), garden_2d_area);

                // println!(
                //     "Garden Area For Plant: {}, Area: {:?}",
                //     node.plant, garden_area
                // );

                result += garden_area.area * garden_area.edges;
            }
        }
    }

    result
}

fn find_similiar(start: Node, garden_2d_area: &mut Vec<Vec<Node>>) -> GardenArea {
    let mut queue = vec![start.clone()];

    let mut garden_area: GardenArea = GardenArea { area: 0, edges: 0 };

    garden_2d_area[start.cord.x][start.cord.y].visited = true;

    while let Some(working_node) = queue.pop() {
        let (new_cords, edges) = get_edge(working_node.clone(), &garden_2d_area);

        garden_area.edges += edges;
        garden_area.area += 1;

        for edge in new_cords {
            let node_pointer = garden_2d_area
                .get_mut(edge.x)
                .unwrap()
                .get_mut(edge.y)
                .unwrap();

            if !node_pointer.visited {
                node_pointer.visited = true;
                queue.push(node_pointer.clone());
            }
        }
    }

    garden_area
}

fn get_edge(start_node: Node, garden_2d_area: &Vec<Vec<Node>>) -> (Vec<Cord>, usize) {
    let mut un_explored_nodes = vec![];
    let mut outside_edge = 4;

    for direction in DIRECTIONS {
        if let Some(new_cord) = new_position_in_direction(&start_node.cord, &direction) {
            if let Some(row) = garden_2d_area.get(new_cord.x) {
                if let Some(new_node) = row.get(new_cord.y) {
                    if new_node.plant == start_node.plant {
                        if !new_node.visited {
                            un_explored_nodes.push(new_node.cord);
                        }
                        outside_edge -= 1;
                    }
                }
            }
        }
    }

    return (un_explored_nodes, outside_edge);
}

#[allow(dead_code)]
fn print_garden_to_array(garden_2d_area: &Vec<Vec<Node>>) {
    println!();
    for node_array in garden_2d_area {
        for node in node_array {
            print!("{}", node.to_string());
        }
        println!();
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct Node {
    pub(crate) plant: char,
    pub(crate) cord: Cord,
    pub(crate) visited: bool,
    pub(crate) edges: usize,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.plant)
    }
}

#[derive(Clone, Debug, Default)]
struct GardenArea {
    area: usize,
    edges: usize,
}

fn content_to_garden_2d_area(input: &str) -> Vec<Vec<Node>> {
    let mut output = vec![];

    let mut working_vec: Vec<Node> = vec![];

    for char in input.chars() {
        if char == '\n' {
            output.push(working_vec.clone());
            working_vec.clear();
        } else {
            working_vec.push(Node {
                plant: char,
                cord: Cord { x: 0, y: 0 },
                visited: false,
                edges: 0,
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

    // let rows = output.len();
    // let cols = output[0].len();
    // let mut bordered_output = vec![
    //     vec![
    //         Node {
    //             plant: '*',
    //             cord: Cord { x: 0, y: 0 },
    //             visited: false,
    //             edges: 0,
    //         };
    //         cols + 2
    //     ];
    //     rows + 2
    // ];

    // for (x, node_array) in output.iter().enumerate() {
    //     for (y, node) in node_array.iter().enumerate() {
    //         bordered_output[x + 1][y + 1] = Node {
    //             plant: node.plant,
    //             cord: Cord { x: x + 1, y: y + 1 },
    //             visited: node.visited,
    //             edges: node.edges,
    //         };
    //     }
    // }

    // bordered_output
}

pub(crate) fn day12_2_garden_groups() {
    println!("Running day12.2 Garden Groups");

    let (content, timer) = start_day("./src/inputs/day12.txt");

    let mut garden_2d_area = content_to_garden_2d_area(&content);

    // print_garden_to_array(&garden_2d_area);

    let result = walk_the_garden(&mut garden_2d_area);

    end_day(&result, &timer);
}
