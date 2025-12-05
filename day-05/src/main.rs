use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;

enum ProcessingMode {
    Range,
    Ingredient
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    let path = Path::new(file_path);
    let file = File::open(path);

    if file.is_err() {
        println!("Unable to read the file {:?}", file_path);
        return;
    }

    let mut processing_mode = ProcessingMode::Range;
    let mut ranges = Vec::new();
    let mut ingredients = Vec::new();

    let reader = BufReader::new(file.unwrap());
    for line in reader.lines() {
        match line {
            Ok(l) => {
                // Strip BOM if present
                let clean = l.trim_start_matches('\u{feff}');

                if clean.is_empty() {
                    // All the ranges have been processed, start processing ingredients.
                    processing_mode = ProcessingMode::Ingredient;
                    continue;
                }

                match processing_mode {
                    ProcessingMode::Range => {
                        let parts = clean.split('-').collect::<Vec<&str>>();
                        let range = std::ops::Range {
                            start: u64::from_str(parts[0].trim()).unwrap(),
                            end: u64::from_str(parts[1].trim()).unwrap() + 1
                        };
                        ranges.push(range);
                    },
                    ProcessingMode::Ingredient => ingredients.push(clean.parse::<u64>().unwrap()),
                }
            },
            Err(_) => continue,
        }
    }

    let fresh_ingredients: Vec<&u64> = ingredients
        .iter()
        .filter(| id | ranges.iter().any(| r | r.contains(id) ) )
        .collect();

    println!("There are {} fresh ingredients.", fresh_ingredients.len());

    ranges.sort_unstable_by_key(| r | r.start);
    
    let mut rationalised_ranges = Vec::new();
    let mut current_range = ranges[0].clone();
    
    for range in ranges.into_iter().skip(1) {
        if range.start <= current_range.end {
            current_range.end = current_range.end.max(range.end);
        } else {
            rationalised_ranges.push(current_range);
            current_range = range;
        }
    }
    
    rationalised_ranges.push(current_range);
    
    println!("There are {} fresh ingredients.", rationalised_ranges.iter().map(| r | r.end - r.start).sum::<u64>());
}

#[cfg(test)]
mod test{
    use super::*;
}