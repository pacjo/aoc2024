use aoc2024::read_into_2d_array;
use array2d::Array2D;

fn safe_get(array: &Array2D<char>, i: i32, j: i32) -> char {
    let default = '.';

    if i < 0 || j < 0 {
        return default;
    }

    if i >= array.num_rows() as i32 || j >= array.num_columns() as i32 {
        return default;
    }

    return *array.get(i as usize, j as usize).unwrap();
}

fn number_of_xmas_matches_in_any_direction(array: &Array2D<char>, i: i32, j: i32) -> i32 {
    // part 1
    return
    	(safe_get(&array, i, j) == 'X' && safe_get(&array, i, j - 1) == 'M' && safe_get(&array, i, j - 2) == 'A' && safe_get(&array, i, j - 3) == 'S') as i32 +					// left
        (safe_get(&array, i, j) == 'X' && safe_get(&array, i - 1, j - 1) == 'M' && safe_get(&array, i - 2, j - 2) == 'A' && safe_get(&array, i - 3, j - 3) == 'S') as i32 +		// upper left
        (safe_get(&array, i, j) == 'X' && safe_get(&array, i - 1, j) == 'M' && safe_get(&array, i - 2, j) == 'A' && safe_get(&array, i - 3, j) == 'S') as i32 +					// up
        (safe_get(&array, i, j) == 'X' && safe_get(&array, i - 1, j + 1) == 'M' && safe_get(&array, i - 2, j + 2) == 'A' && safe_get(&array, i - 3, j + 3) == 'S') as i32 +		// upper right
        (safe_get(&array, i, j) == 'X' && safe_get(&array, i, j + 1) == 'M' && safe_get(&array, i, j + 2) == 'A' && safe_get(&array, i, j + 3) == 'S') as i32 +					// right
        (safe_get(&array, i, j) == 'X' && safe_get(&array, i + 1, j + 1) == 'M' && safe_get(&array, i + 2, j + 2) == 'A' && safe_get(&array, i + 3, j + 3) == 'S') as i32 +		// lower right
        (safe_get(&array, i, j) == 'X' && safe_get(&array, i + 1, j) == 'M' && safe_get(&array, i + 2, j) == 'A' && safe_get(&array, i + 3, j) == 'S') as i32 +					// down
        (safe_get(&array, i, j) == 'X' && safe_get(&array, i + 1, j - 1) == 'M' && safe_get(&array, i + 2, j - 2) == 'A' && safe_get(&array, i + 3, j - 3) == 'S') as i32		// lower left
    ;
}

fn is_there_an_x_max_with_the_center_in(array: &Array2D<char>, i: usize, j: usize) -> bool {
    // part 2
    // in every 3x3 sub grid (for which ww'll be checking) there can only be one match
    //              M.S      S.M  or  M.M      S.S
    // it's either  .A.  or  .A.  or  .A.  or  .A.
    //              S.M      M.S  or  S.S      M.M
    // so let's start by checking if there's an A in the center
    // and then check the corners

    println!(
        "({}, {})\n{}.{}\n.{}.\n{}.{}",
        i,
        j,
        array[(i - 1, j - 1)],
        array[(i - 1, j + 1)],
        array[(i, j)],
        array[(i + 1, j - 1)],
        array[(i + 1, j + 1)]
    );

    if array[(i, j)] != 'A' {
        return false;
    }

    // upper_left upper_right lower_left lower_right
    let combo = format!(
        "{}{}{}{}",
        array[(i - 1, j - 1)],
        array[(i - 1, j + 1)],
        array[(i + 1, j - 1)],
        array[(i + 1, j + 1)]
    );

    println!("combination: {}", combo);

    return vec!["MSMS", "SMSM", "MMSS", "SSMM"].contains(&combo.as_str());
}

fn main() {
    let array = read_into_2d_array("in.txt");

    let mut part1 = 0;

    for i in 0..array.num_rows() {
        for j in 0..array.num_columns() {
            part1 += number_of_xmas_matches_in_any_direction(&array, i as i32, j as i32);
        }
    }

    println!("part 1: {}", part1);

    let mut part2 = 0;

    for i in 1..array.num_rows() - 1 {
        for j in 1..array.num_columns() - 1 {
            println!(
                "matches?: {}\n",
                is_there_an_x_max_with_the_center_in(&array, i, j)
            );
            if is_there_an_x_max_with_the_center_in(&array, i, j) {
                part2 += 1;
            }
        }
    }

    println!("part 2: {}", part2);
}
