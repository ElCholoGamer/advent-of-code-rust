use aoc_lib::{AocSolution, BoxedError, Error};
use aoc_lib::structs::{BidirectionalTree};

#[derive(Debug)]
struct FileSystemItem {
    name: String,
    item_type: ItemType,
}

#[derive(Debug)]
enum ItemType {
    Directory,
    File(u32),
}

fn main() -> Result<(), BoxedError> {
    aoc_lib::run::<Day7>(7)
}

fn calculate_size<F: FnMut(u32)>(tree: &BidirectionalTree<FileSystemItem>, item_index: usize, on_dir: &mut F) -> u32 {
    match &tree[item_index].data.item_type {
        ItemType::File(size) => *size,
        ItemType::Directory => {
            let dir_size = tree.children_of(item_index).iter()
                .map(|&item| calculate_size(tree, item.index, on_dir)).sum();

            on_dir(dir_size);
            dir_size
        }
    }
}

struct Day7;

impl AocSolution for Day7 {
    type Input = BidirectionalTree<FileSystemItem>;
    type Output = u32;

    fn parse_input(raw_input: String) -> Self::Input {
        let mut tree = BidirectionalTree::new(FileSystemItem {
            name: "/".into(),
            item_type: ItemType::Directory,
        });
        let mut current_dir = 0;

        let lines = raw_input.lines().collect::<Vec<_>>();
        let mut i = 1;

        while i < lines.len() {
            let words = lines[i].split_whitespace().collect::<Vec<_>>();
            i += 1;

            match words[1] {
                "cd" => {
                    let destination = words[2];
                    current_dir = if destination == ".." {
                        tree.index_of_parent(current_dir).expect("cannot move up from root")
                    } else {
                        tree.children_of(current_dir).into_iter()
                            .find(|&node| node.data.name == *destination)
                            .map(|node| node.index)
                            .expect("directory not found")
                    };
                }
                "ls" => {
                    while i < lines.len() && !lines[i].starts_with('$') {
                        let (prefix, name) = lines[i].split_once(' ').unwrap();
                        let node = FileSystemItem {
                            name: name.into(),
                            item_type: if prefix == "dir" {
                                ItemType::Directory
                            } else {
                                ItemType::File(prefix.parse().unwrap())
                            },
                        };

                        tree.insert(current_dir, node);
                        i += 1;
                    }
                }
                _ => panic!("invalid command"),
            }
        }

        tree
    }

    fn part_1(tree: &Self::Input) -> Result<Self::Output, BoxedError> {
        let mut total = 0;
        calculate_size(tree, 0, &mut |dir_size| if dir_size <= 100_000 { total += dir_size; }, );
        Ok(total)
    }

    fn part_2(tree: &Self::Input) -> Result<Self::Output, BoxedError> {
        let mut sizes = Vec::new();
        let root_size = calculate_size(tree, 0, &mut |dir_size| sizes.push(dir_size));
        let missing_space = 30_000_000 - (70_000_000 - root_size);

        sizes.sort();
        sizes.into_iter().find(|&size| size >= missing_space)
            .ok_or(Error::Misc("directories too small".into()).into())
    }
}
