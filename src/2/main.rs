use aoc2024::read_lines;

fn is_report_fully_safe(report: Vec<i32>) -> bool {
    // parse reports into diffs
    let mut diffs = Vec::new();
    let mut is_report_safe = true;

    for index in 0..report.len() - 1 {
        diffs.push(report[index + 1] - report[index]);
    }

    println!("\nreport: {:?}\ndiffs: {:?}", report, diffs);

    for diff in &diffs {
        if !((1..=3).contains(diff) || (-3..=-1).contains(diff)) {
            is_report_safe = false;
            println!("diff range check failed! (mismatched diff: {})", diff);

            break;
        }
    }

    if (diffs.iter().max().unwrap() * diffs.iter().min().unwrap()) < 0 {
        is_report_safe = false;
        println!("diffs aren't monotonic!")
    }

    return is_report_safe;
}

fn main() {
    // read file
    let lines = read_lines("in.txt");

    let mut safe_count = 0;
    let mut mostly_safe_count = 0;

    // for every line - report
    for line in lines {
        let mut report = Vec::new();

        let nums = line.split_whitespace();

        for num in nums {
            report.push(num.parse::<i32>().unwrap())
        }

        if is_report_fully_safe(report.clone()) {
            safe_count += 1;
            println!("diff OK!")
        } else {
            // brute-forcing, but it works
            for index in 0..report.len() {
                let mut might_be_safe_report = report.clone();
                might_be_safe_report.remove(index);

                if is_report_fully_safe(might_be_safe_report) {
                    mostly_safe_count += 1;
                    break;
                }
            }
        }
    }

    println!("\n===============\nsafe count: {}", safe_count);
    println!("mostly safe count: {}", safe_count + mostly_safe_count);
}
