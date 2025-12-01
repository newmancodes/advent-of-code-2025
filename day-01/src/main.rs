use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("Applying instructions from {:?}", file_path);
    let path = Path::new(file_path);
    let rotations = Rotation::<u16>::from_file(path.to_str().unwrap());

    if rotations.is_err() {
        println!("Unable to read the file {:?}", file_path);
        return;
    }

    let mut rotation_count = 0;
    let mut dial = Dial::new(50);
    for rotation in rotations.ok().unwrap() {
        dial = dial.rotate(rotation);
        rotation_count += 1;
    }

    println!("The file has {} lines.", rotation_count);
    println!("The dial stopped at 0 {} times.", dial.times_stopped_at_zero);
    println!("The dial passed 0 {} times.", dial.times_passed_zero);
}

#[derive(Debug, PartialEq)]
enum Rotation<T> {
    Left(T),
    Right(T)
}

struct FileNotFoundError;
struct ParseRotationError;

impl FromStr for Rotation<u16> {
    type Err = ParseRotationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let direction = s.chars().next();
        let steps = &s[1..].parse::<u16>();

        match (direction, steps) {
            (Some('L'), Ok(value)) => Ok(Rotation::Left(*value)),
            (Some('R'), Ok(value)) => Ok(Rotation::Right(*value)),
            _ => Err(ParseRotationError)
        }
    }
}

impl Rotation<u16> {
    fn from_file(file_path: &str) -> Result<Vec<Self>, FileNotFoundError> {
        let file = File::open(file_path);
        if file.is_err() {
            return Err(FileNotFoundError);
        }

        let reader = BufReader::new(file.unwrap());

        let mut rotations = Vec::new();
        for line in reader.lines() {
            match line {
                Ok(l) => {
                    // Strip BOM if present
                    let clean = l.trim_start_matches('\u{feff}');
                    match Rotation::<u16>::from_str(clean) {
                        Ok(r) => rotations.push(r),
                        Err(_) => continue,
                    }
                },
                Err(_) => continue,
            }
        }

        Ok(rotations)
    }
}

struct Dial {
    position: u16,
    times_stopped_at_zero: u16,
    times_passed_zero: u16,
}

impl Dial {
    fn new(initial_position: u16) -> Self {
        Self {
            position: initial_position,
            times_stopped_at_zero: 0,
            times_passed_zero: 0,
        }
    }

    fn rotate(mut self, rotation: Rotation<u16>) -> Self {
        let mut new_position = self.position as i32;
        match rotation {
            Rotation::Left(value) => {
                for _ in 0..value {
                    new_position = new_position - 1;
                    if new_position == 0 {
                        self.times_passed_zero += 1;
                    } else if new_position < 0 {
                        new_position = 99;
                    }
                }
            },
            Rotation::Right(value) => {
                for _ in 0..value {
                    new_position = (new_position + 1) % 100;
                    if new_position == 0 {
                        self.times_passed_zero += 1;
                    } else if new_position > 99 {
                        new_position = 0;
                    }
                }
            }
        }
        assert!(new_position >= 0 && new_position < 100, "The new position must be between 0 and 99 (inclusive).");
        self.position = new_position as u16;

        assert!(self.position < 100);

        // If the dial has stopped at 0, increment the counter.
        if self.position == 0 {
            self.times_stopped_at_zero += 1;
        }

        // Return the updated dial.
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotate_right_works() {
        let dial = Dial::new(0);
        let rotated = dial.rotate(Rotation::Right(3));
        assert_eq!(rotated.position, 3);
        assert_eq!(rotated.times_stopped_at_zero, 0);

        let dial = Dial::new(50);
        let rotated = dial.rotate(Rotation::Right(49));
        assert_eq!(rotated.position, 99);
        assert_eq!(rotated.times_stopped_at_zero, 0);

        let dial = Dial::new(99);
        let rotated = dial.rotate(Rotation::Right(1));
        assert_eq!(rotated.position, 0);
        assert_eq!(rotated.times_stopped_at_zero, 1);

        let dial = Dial::new(0);
        let rotated = dial.rotate(Rotation::Right(110));
        assert_eq!(rotated.position, 10);
        assert_eq!(rotated.times_stopped_at_zero, 0);
    }

    #[test]
    fn rotate_left_works() {
        let dial = Dial::new(0);
        let rotated = dial.rotate(Rotation::Left(3));
        assert_eq!(rotated.position, 97);
        assert_eq!(rotated.times_stopped_at_zero, 0);

        let dial = Dial::new(50);
        let rotated = dial.rotate(Rotation::Left(49));
        assert_eq!(rotated.position, 1);
        assert_eq!(rotated.times_stopped_at_zero, 0);

        let dial = Dial::new(1);
        let rotated = dial.rotate(Rotation::Left(1));
        assert_eq!(rotated.position, 0);
        assert_eq!(rotated.times_stopped_at_zero, 1);

        let dial = Dial::new(0);
        let rotated = dial.rotate(Rotation::Left(110));
        assert_eq!(rotated.position, 90);
        assert_eq!(rotated.times_stopped_at_zero, 0);
    }

    #[test]
    fn odd_example_works() {
        let sequence = [Rotation::Left(795)];

        let mut dial = Dial::new(95);

        for rotation in sequence {
            dial = dial.rotate(rotation);
        }

        assert_eq!(dial.times_stopped_at_zero, 1);
    }

    #[test]
    fn simple_example_works() {
        let sequence: [Rotation<u16>; 10] = [
            Rotation::Left(68),
            Rotation::Left(30),
            Rotation::Right(48),
            Rotation::Left(5),
            Rotation::Right(60),
            Rotation::Left(55),
            Rotation::Left(1),
            Rotation::Left(99),
            Rotation::Right(14),
            Rotation::Left(82),
        ];

        let mut dial = Dial::new(50);

        for rotation in sequence {
            dial = dial.rotate(rotation);
        }

        assert_eq!(dial.times_stopped_at_zero, 3);
        assert_eq!(dial.times_passed_zero, 6);
    }

    #[test]
    fn rotation_can_be_parsed()
    {
        let input = "L23";
        let rotation = Rotation::from_str(input);
        assert!(rotation.is_ok_and(|r| r == Rotation::Left(23)));

        let input = "R78";
        let rotation = Rotation::from_str(input);
        assert!(rotation.is_ok_and(|r| r == Rotation::Right(78)));

        let input = "R65536";
        let rotation = Rotation::from_str(input);
        assert!(rotation.is_err_and(|e| matches!(e, ParseRotationError)));

        let input = "Z12";
        let rotation = Rotation::from_str(input);
        assert!(rotation.is_err_and(|e| matches!(e, ParseRotationError)));
    }

    #[test]
    fn file_can_be_parsed()
    {
        println!("{:?}", env::current_dir().unwrap());
        let path = Path::new("./data/example.txt");
        let rotations = Rotation::<u16>::from_file(path.to_str().unwrap());
        assert!(rotations.is_ok());
        assert!(rotations.is_ok_and(|r| r.len() == 10));
    }
}