use aoc2024::read_lines;
use std::iter::repeat;

#[derive(Debug, Copy, Clone, PartialEq)]
enum MemoryType {
    File,
    FreeSpace,
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct MemoryBlock {
    id: usize,
    r#type: MemoryType,
}

#[derive(Debug, Clone, PartialEq)]
struct MemoryItem {
    id: usize,
    blocks: Vec<MemoryBlock>,
}

fn format_memory_item(item: &MemoryItem) -> String {
    let mut out: String = Default::default();

    for block in &item.blocks {
        let string_id = item.id.to_string();

        out += match block.r#type {
            MemoryType::File => &string_id,
            MemoryType::FreeSpace => ".",
        }
    }

    return out;
}

fn format_memory_block(block: &MemoryBlock) -> String {
    return match block.r#type {
        MemoryType::File => block.id.to_string(),
        MemoryType::FreeSpace => '.'.to_string(),
    };
}

fn format_memory_items(memory: &Vec<MemoryItem>) -> String {
    let mut out: String = Default::default();

    for item in memory {
        out += &format_memory_item(item);
    }

    return out;
}

fn format_memory_blocks(memory: &Vec<MemoryBlock>) -> String {
    let mut out: String = Default::default();

    for block in memory {
        out += &format_memory_block(block);
    }

    return out;
}

// TODO: this should probably get blocks instead
fn optimize_memory_blocks(memory: &Vec<MemoryItem>) -> Vec<MemoryBlock> {
    let mut filtered_memory: Vec<MemoryBlock> = memory
        .to_vec()
        .into_iter()
        .map(|item| item.blocks)
        .flatten()
        .filter(|block| block.r#type == MemoryType::File)
        .collect();

    let mut optimized_memory: Vec<MemoryBlock> = Default::default();

    for block in memory.iter().map(|item| item.blocks.clone()).flatten() {
        optimized_memory.push(if filtered_memory.is_empty() {
            MemoryBlock {
                id: block.id,
                r#type: MemoryType::FreeSpace,
            }
        } else if block.r#type == MemoryType::File {
            if !filtered_memory.is_empty() {
                filtered_memory.remove(0);
            }

            block.clone()
        } else if !filtered_memory.is_empty() {
            filtered_memory.pop().unwrap()
        } else {
            unreachable!();
        })
    }

    return optimized_memory;
}

fn optimize_memory_items(memory: &Vec<MemoryItem>) -> Vec<MemoryBlock> {
    let mut filtered_memory: Vec<MemoryItem> = memory
        .to_vec()
        .into_iter()
        .filter(|item| item.blocks.len() > 0)
        .filter(|item| item.blocks[0].r#type == MemoryType::File)
        .collect();

    let mut optimized_memory: Vec<MemoryBlock> = Default::default();

    for item in memory {
        optimized_memory.extend(if filtered_memory.is_empty() {
            let empty_blocks = repeat(MemoryBlock {
                id: item.id,
                r#type: MemoryType::FreeSpace,
            })
            .take(item.blocks.len())
            .collect::<Vec<MemoryBlock>>();

            empty_blocks
        } else if item.blocks[0].r#type == MemoryType::File {
            // this is probably wrong
            if let Some(pos) = filtered_memory.iter().position(|x| x == item) {
                filtered_memory.remove(pos);
            }

            item.blocks.clone()
        } else if !filtered_memory.is_empty() {
            // we could add a file here
            let available_space = item.blocks.len();
            let fitting_items: Vec<MemoryItem> = filtered_memory
                .iter()
                .filter(|x| x.blocks.len() <= available_space)
                .cloned()
                .collect();

            if !fitting_items.is_empty() {
                let fitting_item = fitting_items.last().unwrap(); // this only works if filter preserves order
                filtered_memory.retain(|x| x != fitting_item);

                let remeining_space_blocks_after_swap = repeat(MemoryBlock {
                    id: item.id,
                    r#type: MemoryType::FreeSpace,
                })
                .take(item.blocks.len() - fitting_item.blocks.len())
                .collect::<Vec<MemoryBlock>>();

                fitting_item
                    .blocks
                    .iter()
                    .chain(remeining_space_blocks_after_swap.iter())
                    .cloned()
                    .collect()
            } else {
                // none of the items fit
                repeat(MemoryBlock {
                    id: item.id,
                    r#type: MemoryType::FreeSpace,
                })
                .take(item.blocks.len())
                .collect::<Vec<MemoryBlock>>()
            }
        } else {
            unreachable!();
        })
    }

    return optimized_memory;
}

fn calculate_checksum(memory: &Vec<MemoryBlock>) -> i64 {
    let mut sum: i64 = 0;

    for (index, block) in memory.iter().enumerate() {
        if block.r#type != MemoryType::FreeSpace {
            sum += (index as i64) * (block.id as i64);
        }
    }

    return sum;
}

fn main() {
    let lines = read_lines("test2.txt");
    let memory_line = lines[0].as_str().split("");

    let mut memory: Vec<MemoryItem> = Vec::new();

    // TODO: move into function
    let mut counter = 0;
    for item in memory_line {
        if !item.is_empty() {
            memory.push(MemoryItem {
                id: counter / 2,
                blocks: repeat(MemoryBlock {
                    id: counter / 2,
                    r#type: if counter % 2 == 0 {
                        MemoryType::File
                    } else {
                        MemoryType::FreeSpace
                    },
                })
                .take(item.parse::<usize>().unwrap())
                .collect(),
            });
            counter += 1;
        }
    }

    // println!("memory: {:?}", memory);
    // println!("{}", format_memory_items(&memory));
    // println!("{}", format_memory_blocks(&optimize_memory(&memory)));
    println!(
        "part 1: {}",
        calculate_checksum(&optimize_memory_blocks(&memory))
    );

    println!(
        "part 2: {}",
        calculate_checksum(&optimize_memory_items(&memory))
    );
    // println!("before: {}", format_memory_items(&memory));
    // println!(
    //     "after : {}",
    //     format_memory_blocks(&optimize_memory_items(&memory))
    // );
}
