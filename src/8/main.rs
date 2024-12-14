use std::collections::HashMap;

use aoc2024::{print_2d_array, read_into_2d_array};
use array2d::Array2D;

/// parses array 2d represented map into a list of lists of locations of antenas with the same frequency
fn parse_into_locations(array: &Array2D<char>) -> HashMap<char, Vec<(usize, usize)>> {
    let mut map: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    for row in 0..array.num_rows() {
        for column in 0..array.num_columns() {
            let index = (row, column);

            let frequency = array[index];

            if frequency != '.' {
                map.entry(frequency).or_default().push(index);
            }
        }
    }

    return map;
}

fn generate_pairs<T: Clone>(items: Vec<T>) -> Vec<(T, T)> {
    let mut paired_items: Vec<(T, T)> = Vec::new();

    for i in 0..items.len() {
        for j in 0..items.len() {
            if i != j {
                paired_items.push((items[i].clone(), items[j].clone()));
            }
        }
    }

    return paired_items;
}

fn main() {
    let array = read_into_2d_array("in.txt");
    let mut antinode_array = Array2D::filled_with('.', array.num_rows(), array.num_columns());

    print_2d_array(&array);

    let map = parse_into_locations(&array);
    println!("{:?}", map);

    let mut base_sum = 0;

    for entry in &map {
        for pair in generate_pairs(entry.1.to_vec()) {
            let vector = (
                pair.0 .0 as i32 - pair.1 .0 as i32,
                pair.0 .1 as i32 - pair.1 .1 as i32,
            );

            let location = (pair.1 .0 as i32 - vector.0, pair.1 .1 as i32 - vector.1);

            println!(
                "{:?}, diff vector: {:?}, possible new antinode at: {:?}",
                pair, vector, location
            );

            if (location.0 >= 0 && location.1 >= 0)
                && (location.0 < antinode_array.row_len() as i32
                    && location.1 < antinode_array.column_len() as i32)
            {
                if antinode_array[(location.0 as usize, location.1 as usize)] != '#' {
                    antinode_array[(location.0 as usize, location.1 as usize)] = '#';
                    base_sum += 1;
                }
            }
        }
    }

    print_2d_array(&antinode_array);

    // part 2

    let mut harmonics_sum = 0;

    for entry in map {
        for pair in generate_pairs(entry.1) {
            let vector = (
                pair.0 .0 as i32 - pair.1 .0 as i32,
                pair.0 .1 as i32 - pair.1 .1 as i32,
            );

            let mut location = (pair.1 .0 as i32 - vector.0, pair.1 .1 as i32 - vector.1);

            println!(
                "{:?}, diff vector: {:?}, possible new antinode at: {:?}",
                pair, vector, location
            );

            // one time location bump (should probably be moved inside while loop)
            location = (location.0 as i32 - vector.0, location.1 as i32 - vector.1);

            while (location.0 >= 0 && location.1 >= 0)
                && (location.0 < antinode_array.row_len() as i32
                    && location.1 < antinode_array.column_len() as i32)
            {
                // mark new position and add it to the sum
                if antinode_array[(location.0 as usize, location.1 as usize)] != '#' {
                    antinode_array[(location.0 as usize, location.1 as usize)] = '#';
                    harmonics_sum += 1;
                }

                // bump location
                location = (location.0 as i32 - vector.0, location.1 as i32 - vector.1);
            }

            // ...and the other way around
            location = (location.0 as i32 + vector.0, location.1 as i32 + vector.1);

            while (location.0 >= 0 && location.1 >= 0)
                && (location.0 < antinode_array.row_len() as i32
                    && location.1 < antinode_array.column_len() as i32)
            {
                // mark new position and add it to the sum
                if antinode_array[(location.0 as usize, location.1 as usize)] != '#' {
                    antinode_array[(location.0 as usize, location.1 as usize)] = '#';
                    harmonics_sum += 1;
                }

                // bump location
                location = (location.0 as i32 + vector.0, location.1 as i32 + vector.1);
            }
        }
    }

    print_2d_array(&antinode_array);

    println!("part1: {}", base_sum);
    println!("part2: {}", base_sum + harmonics_sum);
}
