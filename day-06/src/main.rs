use std::env;
use std::ffi::c_int;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
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

    let file = File::open(path);

    let reader = BufReader::new(file.unwrap());
    problems = CephalopodProblem::from_lines(reader.lines().map(| l | l.unwrap())).collect();

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

    fn with_string_operand(self, operand: &str) -> Self {
        let mut new_operands = self.operands;
        for (index, item) in operand.chars().enumerate() {
            if new_operands.is_empty() || new_operands.len() <= index {
                new_operands.push(0u64);
            }

            match item {
                c if c.is_digit(10) => {
                    let digit = c.to_digit(10).unwrap() as u64;
                    new_operands[index] = new_operands[index] * 10 + digit;
                },
                _ => continue,
            }
        }

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

    fn from_lines<I, S>(input: I) -> impl Iterator<Item = CephalopodProblem>
    where
        I : IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let mut operators = Vec::new();
        let mut problem_indexes = Vec::new();

        let lines: Vec<_> = input.into_iter().collect();

        let operator_line = lines.last().unwrap();
        for (index, char) in operator_line.as_ref().chars().enumerate() {
            match char {
                '+' | '*' => {
                    operators.push(char);
                    problem_indexes.push(index);
                },
                _ => continue,
            }
        }

        let mut problems = vec![CephalopodProblem::new(); problem_indexes.len()];

        let mut this_problem_starts_at = 0;

        let mut problem = problems[0].clone();
        let next_problem_starts_at = problem_indexes[1];
        let segment_length = next_problem_starts_at - this_problem_starts_at - 1;
        for line in &lines[0..lines.len()-1] {
            let segment = &line.as_ref()[this_problem_starts_at..this_problem_starts_at + segment_length];
            println!("{:?}", segment);
            problem = problem.with_string_operand(segment);
        }
        let operator = operators[0];
        match operator {
            '+' => problem = problem.add(),
            '*' => problem = problem.multiply(),
            _ => panic!("Unexpected operator {:?}", operator),
        }
        problems[0] = problem;

        for index in [1..problem_indexes.iter().len()] {

        }

        problems.into_iter()
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

    #[test]
    fn an_operand_set_can_be_initialised_from_a_string() {
        let builder = CephalopodProblem::new()
            .with_string_operand("12 ");
        assert_eq!(3, builder.operands.len());
        assert_eq!(1, builder.operands[0]);
        assert_eq!(2, builder.operands[1]);
        assert_eq!(0, builder.operands[2]);
    }

    #[test]
    fn part_two_math_problem_can_be_built() {
        let result = CephalopodProblem::new()
            .with_string_operand("64 ")
            .with_string_operand("23 ")
            .with_string_operand("314")
            .add()
            .result;
        assert_eq!(1_058, result);

        let result = CephalopodProblem::new()
            .with_string_operand("123")
            .with_string_operand(" 45")
            .with_string_operand("  6")
            .multiply()
            .result;
        assert_eq!(8_544, result);
    }

    #[test]
    fn part_two_input_can_be_split_correctly() {
        let input = "988 7   8  8171 71 1      6\n438 83  2  7698 68 8   5827\n318 57  45 1474 71 697 2699\n939 791 53 9839 46 954 4137\n*   +   +  +    *  *   +   ";
        let problems = CephalopodProblem::from_lines(input.lines()).collect::<Vec<_>>();
        assert_eq!(problems.len(), 7);

        assert_eq!(8_889, problems[0].operands[0]);
        assert_eq!(8_313, problems[0].operands[1]);
        assert_eq!(9_439, problems[0].operands[2]);
        assert_eq!(697_487_891_823, problems[0].result);
    }
}