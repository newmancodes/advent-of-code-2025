use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    let path = Path::new(file_path);
    let file = File::open(path);

    if file.is_err() {
        println!("Unable to read the file {:?}", file_path);
        return;
    }

    let reader = BufReader::new(file.unwrap());
    let mut total_output_simple_joltage = 0;
    let mut total_output_complex_joltage = 0u64;

    for line in reader.lines() {
        match line {
            Ok(l) => {
                // Strip BOM if present
                let clean = l.trim_start_matches('\u{feff}');
                total_output_simple_joltage = total_output_simple_joltage + clean.to_string().calculate_simple_joltage();
                total_output_complex_joltage = total_output_complex_joltage + clean.to_string().calculate_complex_joltage();
            },
            Err(_) => continue,
        }
    }

    println!("The total output simple joltage is {}.", total_output_simple_joltage);
    println!("The total output complex joltage is {}.", total_output_complex_joltage);
}

trait JoltageCalculator {
    fn calculate_simple_joltage(&self) -> i32;

    fn calculate_complex_joltage(&self) -> u64;
}

impl JoltageCalculator for String {
    fn calculate_simple_joltage(&self) -> i32 {
        let chars: Vec<char> = self.chars().collect();
        let mut maximum_joltage = 0;

        for i in 0..chars.len() - 1 {
            for j in (i+1)..chars.len() {
                let joltage = chars[i].to_digit(10).unwrap() * 10 + chars[j].to_digit(10).unwrap();
                maximum_joltage = maximum_joltage.max(joltage as i32);
            }
        }

        maximum_joltage
    }

    fn calculate_complex_joltage(&self) -> u64 {
        let chars: Vec<char> = self.chars().collect();
        let mut result = String::new();
        let mut start = 0;

        for position in 0..12 {
            let remaining_needed = 12 - position - 1;

            let search_end = chars.len() - remaining_needed;

            let mut max_digit = '0';
            let mut max_index = start;

            for i in start..search_end {
                if chars[i] > max_digit {
                    max_digit = chars[i];
                    max_index = i;
                }
            }

            result.push(max_digit);
            start = max_index + 1;
        }

        result.parse::<u64>().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_joltage_can_be_calculated() {
        let bank = "987654321111111".to_owned();
        assert_eq!(98, bank.calculate_simple_joltage());

        let bank = "811111111111119".to_owned();
        assert_eq!(89, bank.calculate_simple_joltage());

        let bank = "234234234234278".to_owned();
        assert_eq!(78, bank.calculate_simple_joltage());

        let bank = "818181911112111".to_owned();
        assert_eq!(92, bank.calculate_simple_joltage());
    }

    #[test]
    fn complex_joltage_can_be_calculated() {
        let bank = "987654321111111".to_owned();
        assert_eq!(987654321111u64, bank.calculate_complex_joltage());

        let bank = "811111111111119".to_owned();
        assert_eq!(811111111119u64, bank.calculate_complex_joltage());

        let bank = "234234234234278".to_owned();
        assert_eq!(434234234278u64, bank.calculate_complex_joltage());

        let bank = "818181911112111".to_owned();
        assert_eq!(888911112111u64, bank.calculate_complex_joltage());
    }
}