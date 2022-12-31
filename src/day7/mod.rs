use std::{collections::HashMap, fs};

pub fn main() {
    let contents =
        fs::read_to_string("./src/day7/input.txt").expect("Should have been able to read the file");

    let blocks = contents.split("$ ");

    struct Folder<'a> {
        parent: Option<&'a mut Folder<'a>>,
        folders: HashMap<String, &'a mut Folder<'a>>,
        files: HashMap<String, u32>,
    }

    let file_system = Folder {
        parent: None,
        folders: HashMap::new(),
        files: HashMap::new(),
    };

    let mut current_folder = &mut file_system;

    blocks.skip(2).for_each(|block| {
        let split = block.trim().split('\n').collect::<Vec<_>>();

        println!("{:?}", split);

        if split[0] == "ls" {
            split[1..].iter().for_each(|&listed| {
                let split: Vec<_> = listed.split(" ").collect();

                let part1 = *split.get(0).unwrap();
                let part2 = *split.get(1).unwrap();

                if part1 == "dir" && !current_folder.folders.contains_key(part2) {
                    let folders = &mut current_folder.folders;

                    folders.insert(
                        part2.to_string(),
                        &'a Folder {
                            parent: None,
                            folders: HashMap::new(),
                            files: HashMap::new(),
                        },
                    );
                } else {
                }
            });
        } else if split.len() == 0 && split[0].starts_with("cd ") {
            let folder_name = &split[0][3..].to_string();

            let folder;

            if folder_name == ".." {
                folder = current_folder.parent;
            } else {
                folder = current_folder.folders.get(folder_name).as_mut().unwrap();
            }

            if folder.is_some() {
                current_folder = folder.unwrap();
            }
        }
    });

    println!("> Day 7");

    println!("Part 1: Answer is {}", 0);

    println!("Part 2: Answer is {}", 1);
}
