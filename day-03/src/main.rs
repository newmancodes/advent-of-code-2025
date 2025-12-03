use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    let path = Path::new(file_path);
    let file = File::open(file_path);

    if file.is_err() {
        println!("Unable to read the file {:?}", file_path);
        return;
    }

    let reader = BufReader::new(file.unwrap());
    let mut total_output_joltage = 0;
    for line in reader.lines() {
        match line {
            Ok(l) => {
                // Strip BOM if present
                let clean = l.trim_start_matches('\u{feff}');
                total_output_joltage = total_output_joltage + clean.to_string().calculate_simple_joltage();
            },
            Err(_) => continue,
        }
    }

    println!("The total output simple joltage is {}.", total_output_joltage);
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
        let mut maximum_joltage = 0u64;

        for i in 0..chars.len() - 12 {
            
        }

        maximum_joltage
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