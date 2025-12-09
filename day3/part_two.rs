// this is chatgpt ahh code i wasnt able to do it exactly but still learned a lot
use std::fs;

fn main() {
    let data = fs::read_to_string("batteries.txt").expect("Failed to read file");

    let k = 12; // number of digits to select
    let mut total_joltage: u128 = 0; // use u128 for very large numbers

    for line in data.lines() {
        let digits: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();

        let n = digits.len();
        if n < k {
            println!("Bank has fewer than {} digits, skipping", k);
            continue;
        }

        // Use greedy stack approach
        let mut stack: Vec<u32> = Vec::new();
        let mut to_remove = n - k; // how many digits we can skip

        for &digit in &digits {
            while !stack.is_empty() && to_remove > 0 && stack.last().unwrap() < &digit {
                stack.pop();
                to_remove -= 1;
            }
            stack.push(digit);
        }

        // If stack is longer than k, truncate the extra digits at the end
        let max_digits = &stack[..k];

        // Convert to number
        let joltage: u128 = max_digits.iter().fold(0, |acc, &d| acc * 10 + d as u128);
        total_joltage += joltage;
    }

    println!("Total output joltage: {}", total_joltage);
}
