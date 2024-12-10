use aoc2024::read_lines;

// returns tuple of result and numbers for the equation
fn parse_line(line: String) -> (i64, Vec<i32>) {
    let split_line = line.split(": ").collect::<Vec<&str>>();

    let result = split_line[0].parse::<i64>().unwrap();
    let numbers = split_line[1]
        .split_whitespace()
        .collect::<Vec<&str>>()
        .iter()
        .map(|num| num.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    return (result, numbers);
}

/// generats all possible operator combinations, given a number of required operators
fn generate_operators(n: usize, operators: &Vec<&str>) -> Vec<String> {
    if n == 0 {
        return vec!["".to_string()];
    }

    let shorter_combinations = generate_operators(n - 1, operators);
    let mut new_combinations = Vec::new();

    for combination in shorter_combinations {
        for operator in operators {
            new_combinations.push(combination.clone() + " " + operator);
        }
    }

    return new_combinations;
}

fn test_equation(expected_result: &i64, numbers: &Vec<i32>, operators: &Vec<&str>) -> bool {
    let possible_operators = generate_operators(numbers.len() - 1, operators);

    // println!("{:?}", possible_operators);

    for operator_combination in possible_operators {
        let mut result = numbers[0] as i64;

        for (index, operator) in operator_combination
            .split_whitespace()
            .filter(|operator| operators.contains(operator))
            .collect::<Vec<&str>>()
            .iter()
            .enumerate()
        {
            match *operator {
                "+" => result += numbers[index + 1] as i64,
                "*" => result *= numbers[index + 1] as i64,
                "||" => {
                    result = format!("{}{}", result, numbers[index + 1])
                        .parse::<i64>()
                        .unwrap()
                }

                _ => continue,
            }

            // exit early since we can't lower the number
            if result > *expected_result {
                break;
            }
        }

        // println!("got result: {}, expected: {}", result, expected_result);

        if result == *expected_result {
            return true;
        }
    }

    return false;
}

fn main() {
    let lines = read_lines("in.txt");

    let mut sum1 = 0;
    let part1_operators = vec!["+", "*"];
    let mut failed_lines = Vec::new();

    for (index, line) in lines.iter().enumerate() {
        let parse_result = parse_line(line.to_string());
        let test_result = test_equation(&parse_result.0, &parse_result.1, &part1_operators);
        println!(
            "[{} / {}] {:?} - {}",
            index,
            lines.len(),
            parse_result,
            test_result
        );

        if test_result {
            sum1 += parse_result.0;
        } else {
            failed_lines.push(line);
        }
    }

    println!("\npart1: {}\n", sum1);

    // part 2
    // we only test those lines which failed the first part

    let mut sum2 = sum1;		// start with the sum of correct lines
    let part2_operators = vec!["+", "*", "||"];

    for (index, line) in failed_lines.iter().enumerate() {
        let parse_result = parse_line(line.to_string());
        let test_result = test_equation(&parse_result.0, &parse_result.1, &part2_operators);
        println!(
            "[{} / {}] {:?} - {}",
            index,
            failed_lines.len(),
            parse_result,
            test_result
        );

        if test_result {
            sum2 += parse_result.0;
        }
    }

    println!("\npart2: {}\n", sum2);
}
