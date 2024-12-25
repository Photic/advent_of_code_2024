use std::{collections::HashSet, sync::Mutex, vec};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::utils::get_utility::{
    content_to_2d_array, end_day, print_2d_array, start_day, Cord, Direction,
};

pub(crate) fn day6_guard_gallivant() {
    println!("Running day6 Guard Gallivant");

    let (content, timer) = start_day("./src/inputs/day6.txt");

    let mut current_2d_array = content_to_2d_array(&content, true);

    let mut position = Cord { x: 0, y: 0 };
    let mut current_direction = Direction::Up;
    let mut direction_changed = false;

    loop {
        if position.x == 0 && position.y == 0 {
            for (x, array) in current_2d_array.iter().enumerate() {
                for (y, char) in array.iter().enumerate() {
                    if *char == '^' {
                        position.x = x;
                        position.y = y;
                        break;
                    }
                }
                if position.x != 0 || position.y != 0 {
                    break;
                }
            }
        }

        let mut next_posistion = position;
        solve_position(&mut next_posistion, &current_direction);

        let next_square = *current_2d_array
            .get(next_posistion.x)
            .unwrap()
            .get(next_posistion.y)
            .unwrap();

        if current_direction == Direction::Up {
            match next_square {
                '*' => {
                    stop_moving(&mut current_2d_array, &position);
                    break;
                }
                '#' => {
                    current_direction = Direction::Right;
                    direction_changed = true;
                }
                _ => {
                    current_direction = Direction::Up;
                }
            }
        } else if current_direction == Direction::Right {
            match next_square {
                '*' => {
                    stop_moving(&mut current_2d_array, &position);
                    break;
                }
                '#' => {
                    current_direction = Direction::Down;
                    direction_changed = true;
                }
                _ => {
                    current_direction = Direction::Right;
                }
            }
        } else if current_direction == Direction::Down {
            match next_square {
                '*' => {
                    stop_moving(&mut current_2d_array, &position);
                    break;
                }
                '#' => {
                    current_direction = Direction::Left;
                    direction_changed = true;
                }
                _ => {
                    current_direction = Direction::Down;
                }
            }
        } else if current_direction == Direction::Left {
            match next_square {
                '*' => {
                    stop_moving(&mut current_2d_array, &position);
                    break;
                }
                '#' => {
                    current_direction = Direction::Up;
                    direction_changed = true;
                }
                _ => {
                    current_direction = Direction::Left;
                }
            }
        }

        // println!("Current Direction: {:?}", current_direction);

        if !direction_changed {
            move_position(&mut current_2d_array, &mut position, &current_direction);
        }

        direction_changed = false;

        // print_2d_array(&current_2d_array);
    }

    // print_2d_array(&current_2d_array);

    let mut result = 0;

    for array in current_2d_array {
        for char in array {
            if char == 'X' {
                result += 1;
            }
        }
    }

    end_day(&result, &timer);
}

pub(crate) fn day6_2_guard_gallivant() {
    let (content, timer) = start_day("./src/inputs/day6.txt");

    let mut current_2d_array = content_to_2d_array(&content, true);
    let obstical_2d_array = current_2d_array.clone();

    let mut position = Cord { x: 0, y: 0 };
    let mut current_direction = Direction::Up;

    if position.x == 0 && position.y == 0 {
        for (x, array) in current_2d_array.iter().enumerate() {
            for (y, char) in array.iter().enumerate() {
                if *char == '^' {
                    position.x = x;
                    position.y = y;
                    break;
                }
            }
            if position.x != 0 || position.y != 0 {
                break;
            }
        }
    }

    let mut all_vectors: Vec<(Cord, Direction)> = vec![];

    let mut direction_changed = false;
    let starting_position = position;

    all_vectors.push((position, Direction::Up));

    loop {
        if *current_2d_array
            .get(position.x)
            .unwrap()
            .get(position.y)
            .unwrap()
            == '*'
        {
            break;
        }

        let mut next_position = position;
        solve_position(&mut next_position, &current_direction);

        let next_square = *current_2d_array
            .get(next_position.x)
            .unwrap()
            .get(next_position.y)
            .unwrap();

        if current_direction == Direction::Up {
            match next_square {
                '*' => {
                    stop_moving(&mut current_2d_array, &position);
                    break;
                }
                '#' => {
                    current_direction = Direction::Right;
                    direction_changed = true;
                }
                _ => {
                    current_direction = Direction::Up;
                }
            }
        } else if current_direction == Direction::Right {
            match next_square {
                '*' => {
                    stop_moving(&mut current_2d_array, &position);
                    break;
                }
                '#' => {
                    current_direction = Direction::Down;
                    direction_changed = true;
                }
                _ => {
                    current_direction = Direction::Right;
                }
            }
        } else if current_direction == Direction::Down {
            match next_square {
                '*' => {
                    stop_moving(&mut current_2d_array, &position);
                    break;
                }
                '#' => {
                    current_direction = Direction::Left;
                    direction_changed = true;
                }
                _ => {
                    current_direction = Direction::Down;
                }
            }
        } else if current_direction == Direction::Left {
            match next_square {
                '*' => {
                    stop_moving(&mut current_2d_array, &position);
                    break;
                }
                '#' => {
                    current_direction = Direction::Up;
                    direction_changed = true;
                }
                _ => {
                    current_direction = Direction::Left;
                }
            }
        }

        if !direction_changed {
            move_position(&mut current_2d_array, &mut position, &current_direction);
        }

        direction_changed = false;
        all_vectors.push((position, current_direction.clone()));
    }

    let infinite_loops: Mutex<Vec<(Cord, Vec<Cord>)>> = Mutex::new(vec![]);

    // Goes from 10 seconds to 3 seconds using rayon.
    remove_duplicates_maintain_order(all_vectors.clone())
        .par_iter()
        .for_each(|obstacle_vector| {
            if obstacle_vector.0 != starting_position {
                let mut current_2d_array = obstical_2d_array.clone();

                *current_2d_array
                    .get_mut(obstacle_vector.0.x)
                    .unwrap()
                    .get_mut(obstacle_vector.0.y)
                    .unwrap() = 'O';

                let response =
                    just_keep_swimming(&mut current_2d_array, starting_position, Direction::Up);

                if response.0 >= 2 {
                    let mut infinite_loops = infinite_loops.lock().unwrap();
                    infinite_loops.push((obstacle_vector.0, response.1));
                }
            }
        });

    let unique_loops = remove_duplicate_loops(infinite_loops.into_inner().unwrap());

    // for coordinates in unique_loops.clone() {
    //     print_onto_2d_array(&coordinates.0, &coordinates.1, obstical_2d_array.clone());
    // }

    end_day(&unique_loops.len(), &timer);
}

fn just_keep_swimming(
    current_2d_array: &mut [Vec<char>],
    mut position: Cord,
    mut current_direction: Direction,
) -> (usize, Vec<Cord>) {
    let mut result = 0;

    let mut obstical_direction: Option<Direction> = None;

    let mut loop_positions: HashSet<Cord> = HashSet::new();
    let mut seen_position = 0;

    let mut direction_changed = false;

    loop {
        if obstical_direction.is_some() {
            if loop_positions.contains(&position) {
                seen_position += 1;
            }

            loop_positions.insert(position);
        }

        // Check if next_position is out of bounds
        if *current_2d_array
            .get(position.x)
            .unwrap()
            .get(position.y)
            .unwrap()
            == '*'
        {
            break;
        }

        let mut next_position = position;
        solve_position(&mut next_position, &current_direction);

        let next_square = *current_2d_array
            .get(next_position.x)
            .unwrap()
            .get(next_position.y)
            .unwrap();

        if next_square == 'O' && obstical_direction.is_none() {
            obstical_direction = Some(current_direction.clone());
            loop_positions.insert(position);
        }

        if let Some(o_directoin) = obstical_direction.clone() {
            if next_square == 'O' && o_directoin == current_direction {
                result += 1;
            }
        }

        if seen_position > loop_positions.len() * 2 {
            result = 2;
        }

        if result == 2 {
            break;
        }

        if current_direction == Direction::Up {
            match next_square {
                '*' => {
                    stop_moving(current_2d_array, &position);
                    break;
                }
                '#' => {
                    current_direction = Direction::Right;
                    direction_changed = true;
                }
                'O' => {
                    current_direction = Direction::Right;
                    direction_changed = true;
                }
                _ => {
                    current_direction = Direction::Up;
                }
            }
        } else if current_direction == Direction::Right {
            match next_square {
                '*' => {
                    stop_moving(current_2d_array, &position);
                    break;
                }
                '#' => {
                    current_direction = Direction::Down;
                    direction_changed = true;
                }
                'O' => {
                    current_direction = Direction::Down;
                    direction_changed = true;
                }
                _ => {
                    current_direction = Direction::Right;
                }
            }
        } else if current_direction == Direction::Down {
            match next_square {
                '*' => {
                    stop_moving(current_2d_array, &position);
                    break;
                }
                '#' => {
                    current_direction = Direction::Left;
                    direction_changed = true;
                }
                'O' => {
                    current_direction = Direction::Left;
                    direction_changed = true;
                }
                _ => {
                    current_direction = Direction::Down;
                }
            }
        } else if current_direction == Direction::Left {
            match next_square {
                '*' => {
                    stop_moving(current_2d_array, &position);
                    break;
                }
                '#' => {
                    current_direction = Direction::Up;
                    direction_changed = true;
                }
                'O' => {
                    current_direction = Direction::Up;
                    direction_changed = true;
                }
                _ => {
                    current_direction = Direction::Left;
                }
            }
        }

        if !direction_changed {
            solve_position(&mut position, &current_direction);
        }

        direction_changed = false;
    }

    // println!("Loop Positions: {:?}", loop_positions);
    // print_2d_array(&current_2d_array);
    // println!("Result: {result}");

    return (result, loop_positions.into_iter().collect());
}

fn remove_duplicate_loops(infinite_loops: Vec<(Cord, Vec<Cord>)>) -> Vec<(Cord, Vec<Cord>)> {
    let mut unique_loops = HashSet::new();

    for mut loop_vec in infinite_loops {
        loop_vec.1.sort(); // Sort the inner array
        unique_loops.insert(loop_vec); // Insert the sorted array into the HashSet
    }

    unique_loops.into_iter().collect() // Convert the HashSet back to a Vec
}

fn remove_duplicates_maintain_order(coords: Vec<(Cord, Direction)>) -> Vec<(Cord, Direction)> {
    let mut seen = HashSet::new();
    let mut result = Vec::new();

    for coord in coords {
        if seen.insert(coord.clone()) {
            result.push(coord);
        }
    }

    result
}

#[allow(dead_code)]
fn print_onto_2d_array(
    obstical_position: &Cord,
    positions: &Vec<Cord>,
    mut current_2d_array: Vec<Vec<char>>,
) {
    println!();
    println!("obstacle: {:?}", obstical_position);
    println!("positions: {:?}", positions);
    for cord in positions {
        *current_2d_array
            .get_mut(cord.x)
            .unwrap()
            .get_mut(cord.y)
            .unwrap() = 'X';
    }

    *current_2d_array
        .get_mut(obstical_position.x)
        .unwrap()
        .get_mut(obstical_position.y)
        .unwrap() = 'O';

    print_2d_array(&current_2d_array);
}

fn solve_position(position: &mut Cord, direction: &Direction) {
    match direction {
        Direction::Up => position.x -= 1,
        Direction::Down => position.x += 1,
        Direction::Left => position.y -= 1,
        Direction::Right => position.y += 1,
    }
}

fn stop_moving(current_2d_array: &mut [Vec<char>], position: &Cord) {
    *current_2d_array
        .get_mut(position.x)
        .unwrap()
        .get_mut(position.y)
        .unwrap() = 'X';
}

fn move_position(current_2d_array: &mut [Vec<char>], position: &mut Cord, direction: &Direction) {
    *current_2d_array
        .get_mut(position.x)
        .unwrap()
        .get_mut(position.y)
        .unwrap() = 'X';

    solve_position(position, direction);

    *current_2d_array
        .get_mut(position.x)
        .unwrap()
        .get_mut(position.y)
        .unwrap() = '^';
}

// fn content_to_2d_array(input: &str) -> Vec<Vec<char>> {
//     let mut output = vec![];

//     let mut working_array: Vec<char> = vec![];

//     for char in input.chars() {
//         if char == '\n' {
//             output.push(working_array.clone());
//             working_array.clear();
//         } else {
//             working_array.push(char);
//         }
//     }

//     output.push(working_array);

//     // Add boundary
//     let width = output[0].len();
//     let mut bordered_output = vec![vec!['*'; width + 2]];

//     for row in output {
//         let mut bordered_row = vec!['*'];
//         bordered_row.extend(row);
//         bordered_row.push('*');
//         bordered_output.push(bordered_row);
//     }

//     bordered_output.push(vec!['*'; width + 2]);

//     bordered_output
// }
