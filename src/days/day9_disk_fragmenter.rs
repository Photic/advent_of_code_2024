use crate::utils::get_utility::{end_day, start_day};

pub(crate) fn day9_disk_fragmenter() {
    println!("Running day9 Disk Fragmenter");

    let (content, timer) = start_day("./src/inputs/day9.txt");

    let mut disk_map = content_to_disk_map(&content);

    print_disk_map(&disk_map);

    moving_blocks_left(&mut disk_map);

    // print_disk_map(&disk_map);

    // TOOK WAY TO ONG => 364.612393793s
    end_day(&get_checksum(&disk_map), &timer);
}

fn get_checksum(disk_map: &[DiskBlock]) -> usize {
    let empty_block = ".".to_string();

    let mut result: usize = 0;

    for (id, block) in disk_map.iter().enumerate() {
        if block.content != empty_block {
            result += id * block.content.parse::<usize>().expect("Could not parse int");
        }
    }

    result
}

fn moving_blocks_left(disk_map: &mut [DiskBlock]) {
    let empty_block = ".".to_string();

    let original_disk_map = disk_map.to_owned();
    let mut last_location = 0;

    for i in (0..original_disk_map.len()).rev() {
        let moving_block = original_disk_map.get(i).unwrap();
        if moving_block.content != empty_block {
            for (j, block) in disk_map.to_owned().iter().enumerate().skip(last_location) {
                if block.content == empty_block {
                    *disk_map.get_mut(j).unwrap() = moving_block.clone();
                    *disk_map.get_mut(i).unwrap() = DiskBlock {
                        content: empty_block.clone(),
                    };
                    last_location = j;
                    break;
                }
            }

            if !check_empty_spaces(disk_map, &last_location) {
                break;
            }
        }
    }
}

fn check_empty_spaces(disk_map: &[DiskBlock], last_location: &usize) -> bool {
    let empty_block = ".".to_string();

    for (i, block) in disk_map.iter().enumerate().skip(*last_location) {
        if block.content == empty_block {
            // Found empty space
            for block in disk_map.iter().skip(i) {
                if block.content != empty_block {
                    return true;
                }
            }
        }
    }

    return false;
}

fn print_disk_map(disk_map: &Vec<DiskBlock>) {
    for block in disk_map {
        print!("{}", block.content);
    }
    println!();
}

fn content_to_disk_map(input: &str) -> Vec<DiskBlock> {
    let mut output = vec![];

    let mut current_id = 0;
    let mut file_block = true;

    for char in input.chars() {
        if file_block {
            for _ in 0..char.to_string().parse().expect("Could not parse int") {
                output.push(DiskBlock {
                    content: current_id.to_string(),
                });
            }

            current_id += 1;
            file_block = false;
        } else {
            for _ in 0..char.to_string().parse().expect("Could not parse int") {
                output.push(DiskBlock {
                    content: ".".to_string(),
                });
            }

            file_block = true;
        }
    }

    output
}

pub(crate) fn day9_2_disk_fragmenter() {
    println!("Running day9.2 Disk Fragmenter");

    let (content, timer) = start_day("./src/inputs/day9.txt");

    let mut disk_map_files = content_to_disk_map_whole_files(&content);

    // print_disk_map_whole_files(&disk_map_files);

    move_files_left(&mut disk_map_files);

    // print_disk_map_whole_files(&disk_map_files);

    // So slow it hurts. 729.028477583s
    end_day(&get_checksum_of_files_disk(&disk_map_files), &timer);
}

fn move_files_left(disk_map_files: &mut Vec<FileBlock>) {
    let empty_block = ".".to_string();

    let original_disk_map = disk_map_files.clone();
    for i in (0..original_disk_map.len()).rev() {
        let moving_block = original_disk_map.get(i).unwrap();

        if moving_block.content != empty_block {
            for (j, block) in disk_map_files.clone().iter().enumerate() {
                if block.size >= moving_block.size && block.content == empty_block && j < i {
                    disk_map_files.get_mut(i).unwrap().content = empty_block.clone();

                    let remaining: i32 = (block.size as i32 - moving_block.size as i32).abs();

                    {
                        let place_file = disk_map_files.get_mut(j).unwrap();
                        place_file.content = moving_block.content.clone();
                        place_file.size = moving_block.size;
                    }

                    disk_map_files.insert(
                        j + 1,
                        FileBlock {
                            content: empty_block,
                            size: remaining as usize,
                        },
                    );

                    return move_files_left(disk_map_files);
                }
            }
        }
    }
}

fn get_checksum_of_files_disk(disk_map_files: &Vec<FileBlock>) -> usize {
    let empty_block = ".".to_string();

    let mut result: usize = 0;

    let mut disk_map: Vec<DiskBlock> = vec![];

    // Disk map files to disk_map
    for block in disk_map_files {
        for _ in 0..block.size {
            disk_map.push(DiskBlock {
                content: block.content.clone(),
            });
        }
    }

    for (id, block) in disk_map.iter().enumerate() {
        if block.content != empty_block {
            result += id * block.content.parse::<usize>().expect("Could not parse int");
        }
    }

    result
}

fn content_to_disk_map_whole_files(input: &str) -> Vec<FileBlock> {
    let mut output = vec![];

    let mut current_id = 0;
    let mut file_block = true;

    for char in input.chars() {
        if file_block {
            output.push(FileBlock {
                content: current_id.to_string(),
                size: char.to_string().parse().expect("Could not parse int"),
            });
            current_id += 1;
            file_block = false;
        } else {
            output.push(FileBlock {
                content: ".".to_string(),
                size: char.to_string().parse().expect("Could not parse int"),
            });
            file_block = true;
        }
    }

    output
}

#[allow(dead_code)]
fn print_disk_map_whole_files(disk_map: &Vec<FileBlock>) {
    for file in disk_map {
        for _ in 0..file.size {
            print!("{}", file.content);
        }
    }
    println!();
}

#[derive(Debug, Clone)]
struct DiskBlock {
    content: String,
}

#[derive(Debug, Clone)]
struct FileBlock {
    content: String,
    size: usize,
}
