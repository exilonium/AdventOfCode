use std::fs;

fn main() {
    let data = fs::read_to_string("batteries.txt").expect("Failed to read file");

    let mut total_joltage = 0;

    for line in data.lines() {
        let digits: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();

        let mut max_joltage = 0;

        for i in 0..digits.len() {
            for j in i + 1..digits.len() {
                // only consider i < j to preserve order
                let joltage = digits[i] * 10 + digits[j];
                if joltage > max_joltage {
                    max_joltage = joltage;
                }
            }
        }

        total_joltage += max_joltage;
    }

    println!("Total output joltage: {}", total_joltage);
}
