use array2d::Array2D;
use std::fs::read_to_string;

pub fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    return result;
}

pub fn read_into_2d_array(filename: &str) -> Array2D<char> {
    return Array2D::from_rows(
        &read_lines(filename)
            .iter()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>(),
    )
    .unwrap();
}

pub fn print_2d_array(array: &Array2D<char>) {
    for line in array.as_rows() {
        println!("{}", line.iter().collect::<String>());
    }
}

pub fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}
