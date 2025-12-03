use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    let path = Path::new(file_path);
    let product_id_ranges = ProductIdRange::from_file(path.to_str().unwrap());

    if product_id_ranges.is_err() {
        println!("Unable to read the file {:?}", file_path);
        return;
    }

    let invalid_product_id_sum = product_id_ranges.unwrap().into_iter()
        .flat_map(| r | r.into_iter())
        .filter(| p | p.repeats_sequence_of_digits())
        .sum::<u64>();

    println!("The file has and invalid product id sum of {}.", invalid_product_id_sum);
}

#[derive(Debug)]
struct FileNotFoundError;
struct RangeParsingError;

#[derive(Debug, PartialEq)]
struct ProductIdRange {
    from: u64,
    to: u64
}

impl ProductIdRange {
    fn new(from: u64, to: u64) -> Self {
        assert!(from <= to);

        Self { from, to }
    }

    fn from_file(file_path: &str) -> Result<Vec<ProductIdRange>, FileNotFoundError> {
        let file = File::open(file_path);
        if file.is_err() {
            return Err(FileNotFoundError);
        }

        let mut reader = BufReader::new(file.unwrap());
        let mut line = String::new();
        let read = reader.read_line(&mut line);

        if read.is_ok() {
            let mut product_id_ranges = Vec::new();

            // Strip BOM if present
            let clean = line.trim_start_matches('\u{feff}');
            for raw_product_id_range in clean.split(',') {
                match ProductIdRange::from_str(raw_product_id_range) {
                    Ok(product_id_range) => {
                        product_id_ranges.push(product_id_range);
                    },
                    Err(_) => continue
                }
            }

            return Ok(product_id_ranges);
        }

        Err(FileNotFoundError)
    }
}

impl FromStr for ProductIdRange {
    type Err = RangeParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('-').collect();

        if parts.len() != 2 {
            return Err(RangeParsingError);
        }

        let from = parts[0].parse::<u64>().map_err(|_| RangeParsingError)?;
        let to = parts[1].parse::<u64>().map_err(|_| RangeParsingError)?;

        Ok(Self::new(from, to))
    }
}

impl IntoIterator for ProductIdRange {
    type Item = u64;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        (self.from..=self.to).collect::<Vec<u64>>().into_iter()
    }
}

trait RepeatedDigitSequenceDetector {
    fn repeats_sequence_of_digits_twice(self) -> bool;

    fn repeats_sequence_of_digits(self) -> bool;
}

impl RepeatedDigitSequenceDetector for u64 {
    fn repeats_sequence_of_digits_twice(self) -> bool {
        let s = self.to_string();
        if s.len() % 2 == 1 {
            // A string must have an even length to contain a repeated digit sequence.
            return false;
        }

        let (first, last) = s.split_at(s.len() / 2);
        first.parse::<u64>().unwrap() == last.parse::<u64>().unwrap()
    }

    fn repeats_sequence_of_digits(self) -> bool {
        let s = self.to_string();
        let clean_divisors = match s.len() {
            0 | 1 => vec![1],
            2 => vec![1, 2],
            3 => vec![1, 3],
            4 => vec![1, 2, 4],
            5 => vec![1, 5],
            6 => vec![1, 2, 3, 6],
            7 => vec![1, 7],
            8 => vec![1, 2, 4, 8],
            9 => vec![1, 3, 9],
            10 => vec![1, 2, 5, 10],
            _ => vec![],
        };
        
        for divisor in clean_divisors {
            let segments: Vec<&str> = s.as_bytes()
                .chunks(divisor)
                .map(| chunk | std::str::from_utf8(chunk).unwrap())
                .collect();

            if segments.windows(2).all(| w | w[0] == w[1]) && segments.len() > 1 {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn product_id_ranges_can_be_parsed() {
        let s = "11-22";
        let product_id_range = ProductIdRange::from_str(s);
        assert!(product_id_range.is_ok());
        let expected = ProductIdRange { from: 11, to: 22 };
        assert_eq!(expected, product_id_range.ok().unwrap());

        let s = "11-";
        let product_id_range = ProductIdRange::from_str(s);
        assert!(product_id_range.is_err());

        let s = "11-22-33";
        let product_id_range = ProductIdRange::from_str(s);
        assert!(product_id_range.is_err());
    }

    #[test]
    fn product_id_ranges_can_be_enumerated() {
        let r = ProductIdRange::new(1, 1);
        let product_ids: Vec<u64> = r.into_iter().collect();
        assert_eq!(product_ids, vec![1]);

        let r = ProductIdRange::new(11, 22);
        let product_ids: Vec<u64> = r.into_iter().collect();
        assert_eq!(product_ids, vec![11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22]);
    }

    #[test]
    fn u64_can_detect_repeated_digit_sequences() {
        let n = 11;
        assert!(n.repeats_sequence_of_digits_twice());

        let n = 1010;
        assert!(n.repeats_sequence_of_digits_twice());

        let n = 1188511885;
        assert!(n.repeats_sequence_of_digits_twice());

        let n = 10;
        assert!(!n.repeats_sequence_of_digits_twice());

        let n = 644446;
        assert!(!n.repeats_sequence_of_digits_twice());
    }

    #[test]
    fn u64_can_detect_repeated_digit_sequences_of_varying_lengths() {
        let n = 11;
        assert!(n.repeats_sequence_of_digits());

        let n = 22;
        assert!(n.repeats_sequence_of_digits());

        let n = 999;
        assert!(n.repeats_sequence_of_digits());

        let n = 824824824;
        assert!(n.repeats_sequence_of_digits());
    }

    #[test]
    fn simple_example_works() {
        let product_id_ranges = vec![
            "11-22",
            "95-115",
            "998-1012",
            "1188511880-1188511890",
            "222220-222224",
            "1698522-1698528",
            "446443-446449",
            "38593856-38593862",
            "565653-565659",
            "824824821-824824827",
            "2121212118-2121212124"
        ];

        let result = product_id_ranges.iter()
            .map(| s | ProductIdRange::from_str(s))
            .filter_map(| r | r.ok())
            .flat_map(| r | r.into_iter())
            .filter(| p | p.repeats_sequence_of_digits_twice())
            .sum::<u64>();

        assert_eq!(1227775554, result);
    }

    #[test]
    fn simple_example_part_two_works() {
        let product_id_ranges = vec![
            "11-22",
            "95-115",
            "998-1012",
            "1188511880-1188511890",
            "222220-222224",
            "1698522-1698528",
            "446443-446449",
            "38593856-38593862",
            "565653-565659",
            "824824821-824824827",
            "2121212118-2121212124"
        ];

        let result = product_id_ranges.iter()
            .map(| s | ProductIdRange::from_str(s))
            .filter_map(| r | r.ok())
            .flat_map(| r | r.into_iter())
            .filter(| p | p.repeats_sequence_of_digits())
            .sum::<u64>();

        assert_eq!(4174379265, result);
    }
}