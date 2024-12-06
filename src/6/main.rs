use aoc2024::{print_2d_array, read_into_2d_array};
use array2d::Array2D;

static GUARD_CHARACTERS: [char; 4] = ['>', 'v', '<', '^'];
static BLOCK: char = '#';
static VISITED: char = 'X';

struct Guard {
    x: usize,
    y: usize,
    char: char,
}

impl Guard {
    fn new(x: usize, y: usize, char: char) -> Result<Self, String> {
        if GUARD_CHARACTERS.contains(&char) {
            Ok(Guard { x, y, char })
        } else {
            Err(format!("Invalid guard character: {}", char))
        }
    }
}

/// returns array after the next step or None when guard is out-of-bounds
fn next_step(array: &Array2D<char>) -> Option<Array2D<char>> {
    let mut next_array = array.clone();

    let guard = get_guard_info(&array).unwrap();

    let next_position_offset: (i32, i32) = match guard.char {
        c if c == GUARD_CHARACTERS[0] => (0, 1),
        c if c == GUARD_CHARACTERS[1] => (1, 0),
        c if c == GUARD_CHARACTERS[2] => (0, -1),
        c if c == GUARD_CHARACTERS[3] => (-1, 0),

        _ => panic!("this can't happen"),
    };

    // new (possibly invalid) position
    let new_x = guard.x as i32 + next_position_offset.0;
    let new_y = guard.y as i32 + next_position_offset.1;

    // println!("old pos: ({}, {})", guard.x, guard.y);
    // println!("new pos: ({}, {})", new_x, new_y);
    // println!(
    //     "ranges: ({:?}, {:?})",
    //     (0..array.row_len() as i32),
    //     (0..array.column_len() as i32)
    // );

    // guard out-of-bounds
    if !((0..array.row_len() as i32).contains(&new_x))
        || !((0..array.column_len() as i32).contains(&new_y))
    {
        println!("Guard out of bounds!");
        return None;
    }

    // in-bounds, before a blockage
    if array[((new_x) as usize, (new_y) as usize)] == BLOCK {
        let next_guard_char = GUARD_CHARACTERS[(GUARD_CHARACTERS
            .iter()
            .position(|&char| char == guard.char)
            .unwrap()
            + 1)
            % GUARD_CHARACTERS.len()];

        println!(
            "Guard blocked, rotating! ({} ==> {})",
            guard.char, next_guard_char
        );

        next_array[(guard.x, guard.y)] = next_guard_char;

        return Some(next_array);
    }

    // in-bounds, can move forward
    // convert current position into VISITED char
    next_array[(guard.x, guard.y)] = VISITED;

    // move to a new position
    next_array[(new_x as usize, new_y as usize)] = guard.char;

    return Some(next_array);
}

fn get_guard_info(array: &Array2D<char>) -> Option<Guard> {
    for (x, line) in array.as_rows().iter().enumerate() {
        for (y, &char) in line.iter().enumerate() {
            if GUARD_CHARACTERS.contains(&char) {
                if let Ok(guard) = Guard::new(x, y, char) {
                    return Some(guard);
                }
            }
        }
    }

    return None;
}

fn main() {
    let mut array = read_into_2d_array("in.txt");
    print_2d_array(&array);

    while let Some(next) = next_step(&array) {
        array = next;
        // println!();
        // print_2d_array(&array);
    }

    print_2d_array(&array);

    println!(
        "part 1: {}",
        array
            .as_rows()
            .concat()
            .iter()
            .filter(|c| c == &&VISITED || GUARD_CHARACTERS.contains(c))
            .count()
    );
}
