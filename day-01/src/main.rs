use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::collections::BTreeSet;

fn main() {
    // let input = File::open("input.txt");
    // let buffered = BufReader::new(input);

    let buffered = match File::open("input.txt") {
        Ok(file) => BufReader::new(file),
        Err(error) => panic!("File open error: {:?}", error),
    };

    let mut current_calories: i32 = 0;
    let mut max_calories = BTreeSet::new();

    for line_result in buffered.lines() {
        let line = line_result.unwrap();
        if line == "" {
            max_calories.insert(current_calories);
            if max_calories.len() > 3 {
                max_calories.pop_first();
            }
            current_calories = 0;
        } else {
            current_calories += line.parse::<i32>().unwrap();
        }
    }

    let mut total_cals = 0;
    for cals in max_calories {
        total_cals += cals;
    }
    println!("Total calories: {}", total_cals);
}
