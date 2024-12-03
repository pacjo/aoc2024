use aoc2024::read_lines;
use regex::Regex;

fn main() {
    let lines = read_lines("in.txt");

    let mul_regex = r"(mul\([0-9]{1,3},[0-9]{1,3}\))";
    let do_regex = r"(do\(\))";
    let dont_regex = r"(don't\(\))";

    let conditional_mul_regex =
        Regex::new(&format!("{mul_regex}|{do_regex}|{dont_regex}")).unwrap();
    let inputs_regex = Regex::new(r"[0-9]{1,3}").unwrap();

    let mut basic_sum = 0;
    let mut conditional_sum = 0;

    let mut multiplication_enabled = true;

    for memory in lines {
        for regex_match in conditional_mul_regex.find_iter(&memory) {
            if Regex::new(mul_regex)
                .unwrap()
                .is_match(regex_match.as_str())
            {
                let mut inputs = inputs_regex.find_iter(regex_match.as_str());

                let a = inputs.next().unwrap().as_str().parse::<i32>().unwrap();
                let b = inputs.next().unwrap().as_str().parse::<i32>().unwrap();

                println!(
                    "\nmatch: {}\ninputs:\n\t- {}\n\t- {}\nresult: {}",
                    regex_match.as_str(),
                    a,
                    b,
                    a * b
                );

                basic_sum += a * b;

                if multiplication_enabled {
                    conditional_sum += a * b;
                }
            } else if Regex::new(do_regex).unwrap().is_match(regex_match.as_str()) {
                multiplication_enabled = true;
            } else if Regex::new(dont_regex)
                .unwrap()
                .is_match(regex_match.as_str())
            {
                multiplication_enabled = false;
            }
        }
    }

    println!("\n==========================\nsum: {}", basic_sum);
    println!("conditional sum: {}", conditional_sum);
}
