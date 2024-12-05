// use aoc2024::print_type_of;
use std::fs::read_to_string;

/// returns the index of the page that failed the check and the corresponding rule
fn check_update<'a>(update: &str, rules: &Vec<&'a str>) -> (i32, Option<&'a str>) {
    for page in update.split(",") {
        let interesting_rules = rules
            .iter()
            .filter(|rule| rule[..rule.find("|").unwrap()].contains(page));

        // println!("page: {}, rules for page:", page);
        for interesting_rule in interesting_rules {
            // println!(
            //     "\t- {}, ok?: {}",
            //     interesting_rule,
            //     !update[..update.find(page).unwrap()]
            //         .contains(&interesting_rule[interesting_rule.find("|").unwrap() + 1..])
            // );

            if update[..update.find(page).unwrap()]
                .contains(&interesting_rule[interesting_rule.find("|").unwrap() + 1..])
            {
                return (update.find(page).unwrap() as i32, Some(interesting_rule));
            }
        }
    }
    // println!();

    return (-1, None);
}

// curious as to what does the 'a do
fn fix_update<'a>(update: &'a str, rules: &Vec<&str>) -> String {
    let mut fixed_update: String = update.to_string();

    let mut fail_info = check_update(update, rules);

    // hacky do-while
    while fail_info.0 != -1 {
        // println!("fail info: {:?}", fail_info);
        let broken_rule = fail_info.1.unwrap();
        let the_other_value = &broken_rule[broken_rule.find("|").unwrap() + 1..];

        // println!(
        //     "{} broke rule: {} with the other value being: {}",
        //     &fixed_update[(fail_info.0 as usize)..(fail_info.0 as usize) + 2], // fuck it, I'm hard coding +2 (might change it to find someday)
        //     fail_info.1.unwrap(),
        //     the_other_value
        // );

        fixed_update = format!(
            "{}{}{}{}{}",
            &fixed_update[..fixed_update.find(the_other_value).unwrap()], // before the first swapped value
            &fixed_update[fail_info.0 as usize..fail_info.0 as usize + 2], // first swapped value
            // "",
            &fixed_update[fixed_update.find(the_other_value).unwrap() + 2..fail_info.0 as usize], // between the first swapped value and the second
            the_other_value, // second swapped value
            &fixed_update[fail_info.0 as usize + 2..]
        );
        fail_info = check_update(&fixed_update, rules);
    }

    return fixed_update.to_string();
}

fn find_middle_page_number(update: &str) -> i32 {
    let pages: Vec<&str> = update.split(",").collect();

    return pages[pages.len() / 2].parse::<i32>().unwrap();
}

fn main() {
    let file = read_to_string("in.txt");

    let whole_file = file.unwrap();
    let parts: Vec<&str> = whole_file.split("\n\n").collect();

    // rules
    let mut rules = Vec::new();
    for line in parts[0].lines() {
        rules.push(line);
    }

    println!("rules: {:?}", rules);

    // updates
    let mut updates = Vec::new();
    for line in parts[1].lines() {
        updates.push(line);
    }

    println!("rules: {:?}", updates);

    let mut sum_of_page_numbers = 0;
    let mut sum_of_page_numbers_of_broken = 0;

    for update in updates {
        println!("\nchecking update: {}", update);

        let result = check_update(update, &rules);
        if result.0 == -1 {
            println!("result OK!");

            println!("middle page: {}", find_middle_page_number(update));
            sum_of_page_numbers += find_middle_page_number(update);
        } else {
            println!("result NOT OK.");

            let fixed_update = fix_update(update, &rules);
            println!("fixed: {}", fixed_update);
            sum_of_page_numbers_of_broken += find_middle_page_number(&fixed_update);
        }
    }

    println!("part 1: {}", sum_of_page_numbers);
    println!("part 2: {}", sum_of_page_numbers_of_broken);
}
