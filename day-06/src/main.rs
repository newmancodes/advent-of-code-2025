use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;

#[derive(PartialEq)]
enum ProcessingMode {
    Operands,
    Operators
}

enum Operator {
    Add,
    Multiply
}

impl FromStr for Operator {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operator::Add),
            "*" => Ok(Operator::Multiply),
            _ => Err(format!("Invalid operator: {}", s))
        }
    }
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

    let mut problems = Vec::new();
    let mut processing_mode = ProcessingMode::Operands;

    let reader = BufReader::new(file.unwrap());
    for line in reader.lines() {
        match line {
            Ok(l) => {
                // Strip BOM if present
                let cleaned_items: Vec<&str> = l.trim_start_matches('\u{feff}').split_whitespace().collect();

                if cleaned_items[0] == "+" || cleaned_items[0] == "*" {
                    println!("Switching to Operators mode");
                    processing_mode = ProcessingMode::Operators;
                }

                match processing_mode {
                    ProcessingMode::Operands => {
                        let operands: Vec<u64> = cleaned_items.iter().map(|s| s.parse::<u64>().unwrap()).collect();
                        for (index, operand) in operands.iter().enumerate() {
                            if problems.is_empty() || problems.len() <= index {
                                problems.push(CephalopodProblem::new().with_operand(*operand));
                            } else {
                                let problem = problems.get(index).unwrap();
                                problems[index] = problem.clone().with_operand(*operand);
                            }
                        }
                    },
                    ProcessingMode::Operators => {
                        let operators: Vec<Operator> = cleaned_items.iter().map(|s| Operator::from_str(s).unwrap()).collect();
                        for (index, operator) in operators.iter().enumerate() {
                            let problem = problems.get(index).unwrap();
                            problems[index] = match operator {
                                Operator::Add => problem.clone().add(),
                                Operator::Multiply => problem.clone().multiply(),
                            }
                        }
                    },
                }
            },
            Err(_) => continue,
        }
    }
    
    println!("Result: {}", problems.iter().map(| p | p.result).sum::<u64>());
}

#[derive(Clone)]
struct CephalopodProblem {
    operands: Vec<u64>,
    result: u64
}

impl CephalopodProblem {
    fn new() -> Self {
        Self {
            operands: Vec::new(),
            result: 0
        }
    }

    fn with_operand(self, operand: u64) -> Self {
        let mut new_operands = self.operands;
        new_operands.push(operand);
        Self {
            operands: new_operands,
            result: self.result
        }
    }

    fn multiply(self) -> Self {
        let product = self.operands.iter().product();
        Self {
            operands: self.operands,
            result: product
        }
    }

    fn add(self) -> Self {
        let sum = self.operands.iter().sum();
        Self {
            operands: self.operands,
            result: sum
        }
    }
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn math_problem_can_be_built() {
        let result = CephalopodProblem::new()
            .with_operand(328)
            .with_operand(64)
            .with_operand(98)
            .add()
            .result;
        assert_eq!(490, result);

        let result = CephalopodProblem::new()
            .with_operand(123)
            .with_operand(45)
            .with_operand(6)
            .multiply()
            .result;
        assert_eq!(33_210, result);
    }
}