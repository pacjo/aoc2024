use aoc2024::read_lines;

fn main() {
    let lines = read_lines("in.txt");

    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();

    for line in lines {
        let mut nums = line.split_whitespace();

        let first = nums.next().unwrap().parse::<i32>().unwrap();
        let second = nums.next().unwrap().parse::<i32>().unwrap();

        vec1.push(first);
        vec2.push(second);
    }

    // sort both
    vec1.sort();
    vec2.sort();

    let mut distance = 0;

    // calculate distance
    for (first, second) in vec1.iter().zip(vec2.iter()) {
        distance += (second - first).abs();
    }

    println!("distance: {}", distance);

    let mut similarity = 0;

    for num in vec1 {
        similarity += num * vec2.iter().filter(|&n| *n == num).count() as i32;
    }

    println!("similarity: {}", similarity);
}
