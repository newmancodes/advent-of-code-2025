use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;

struct BeamRow {
    beam_positions: Vec<usize>
}

impl Default for BeamRow {
    fn default() -> Self {
        BeamRow { beam_positions: vec![] }
    }
}

enum Node {
    StartPosition,
    Splitter
}

struct NodeRow {
    nodes: Vec<Option<Node>>
}

impl NodeRow {
    fn len(&self) -> usize {
        self.nodes.len()
    }
}

#[derive(Debug)]
struct UnexpectedCharacterError;

impl FromStr for NodeRow {
    type Err = UnexpectedCharacterError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut row = Vec::new();
        for char in s.chars() {
            match char {
                '.' => row.push(None),
                '^' => row.push(Some(Node::Splitter)),
                'S' => row.push(Some(Node::StartPosition)),
                _ => return Err(UnexpectedCharacterError),
            }
        }

        Ok(NodeRow { nodes: row })
    }
}

struct NodeGrid {
    rows: Vec<NodeRow>,
}

impl NodeGrid {
    fn determine_split_and_path_counts(&self) -> (i32, i32) {
        let mut path_count = 0;
        let mut split_count = 0;
        let mut beam_row = BeamRow::default();

        for row in &self.rows {
            let mut new_beam_positions = vec![];

            for (index, node_option) in row.nodes.iter().enumerate() {
                match node_option {
                    Some(Node::StartPosition) => {
                        path_count = path_count + 1;
                        new_beam_positions.push(index)
                    },
                    Some(Node::Splitter) => {
                        // Split the beam if a beam fed into this splitter (based on column index).
                        if beam_row.beam_positions.contains(&index) {
                            if &index > &0 {
                                if !new_beam_positions.contains(&(index - 1)) {
                                    new_beam_positions.push(index - 1);
                                }
                            }

                            if index + 1 < row.nodes.len() {
                                new_beam_positions.push(index + 1);
                            }

                            split_count = split_count + 1;
                        }
                    },
                    None => {
                        if beam_row.beam_positions.contains(&index) {
                            new_beam_positions.push(index);
                        }
                    }
                }
            }

            beam_row.beam_positions = new_beam_positions;
        }

        (split_count, path_count)
    }

    fn from_input<I, S>(input: I) -> Result<NodeGrid, UnexpectedCharacterError>
    where
        I : IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let mut grid_width = 0;
        let mut rows = Vec::new();

        for item in input {
            let row = NodeRow::from_str(item.as_ref())?;
            grid_width = grid_width.max(row.len());
            rows.push(row);
        }

        Ok(NodeGrid { rows })
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

    let reader = BufReader::new(file.unwrap());
    match NodeGrid::from_input(reader.lines().map(| l | l.unwrap().trim_start_matches('\u{feff}').to_owned())) {
        Ok(grid) => {
            let split_count = grid.determine_split_and_path_counts();
            println!("The beam was split {} times.", split_count.0);
            println!("There are {} paths through the beam.", split_count.1);
        },
        Err(e) => panic!(
            "Unable to parse the input file {:?}: {:?}",
            file_path, e
        )
    }
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn example_with_zero_splitters_has_zero_splits_and_one_path()
    {
        let input = vec![
            "..S..",
            ".....",
            ".....",
            ".....",
            "....."
        ];

        let grid = NodeGrid::from_input(input);
        assert!(grid.is_ok());
        let (split_count, path_count) = grid.unwrap().determine_split_and_path_counts();

        assert_eq!(split_count, 0);
        // assert_eq!(path_count, 1);
    }

    #[test]
    fn example_with_unencountered_splitters_has_zero_split_and_one_path()
    {
        let input = vec![
            "..S..",
            "...^.",
            ".^...",
            "....^",
            "^...."
        ];

        let grid = NodeGrid::from_input(input);
        assert!(grid.is_ok());
        let (split_count, path_count) = grid.unwrap().determine_split_and_path_counts();

        assert_eq!(split_count, 0);
        assert_eq!(path_count, 1);
    }

    #[test]
    fn example_with_one_encountered_splitter_has_one_split_and_two_paths()
    {
        let input = vec![
            "..S..",
            ".....",
            "..^..",
            ".....",
            "....."
        ];

        let grid = NodeGrid::from_input(input);
        assert!(grid.is_ok());
        let (split_count, path_count) = grid.unwrap().determine_split_and_path_counts();

        assert_eq!(split_count, 1);
        assert_eq!(path_count, 2);
    }

    #[test]
    fn example_with_two_encountered_splitters_has_two_splits_and_three_paths()
    {
        let input = vec![
            "..S..",
            ".....",
            "..^..",
            ".^...",
            "....."
        ];

        let grid = NodeGrid::from_input(input);
        assert!(grid.is_ok());
        let (split_count, path_count) = grid.unwrap().determine_split_and_path_counts();

        assert_eq!(split_count, 2);
        assert_eq!(path_count, 3);
    }

    #[test]
    fn example_with_three_encountered_splitters_has_three_splits_and_four_paths()
    {
        let input = vec![
            "..S..",
            ".....",
            "..^..",
            ".^...",
            "...^."
        ];

        let grid = NodeGrid::from_input(input);
        assert!(grid.is_ok());
        let (split_count, path_count) = grid.unwrap().determine_split_and_path_counts();

        assert_eq!(split_count, 3);
        assert_eq!(path_count, 4);
    }

    #[test]
    fn example_has_twenty_one_splits_and_forty_paths()
    {
        let input = vec![
            ".......S.......",
            "...............",
            ".......^.......",
            "...............",
            "......^.^......",
            "...............",
            ".....^.^.^.....",
            "...............",
            "....^.^...^....",
            "...............",
            "...^.^...^.^...",
            "...............",
            "..^...^.....^..",
            "...............",
            ".^.^.^.^.^...^.",
            "...............",
        ];

        let grid = NodeGrid::from_input(input);
        assert!(grid.is_ok());
        let (split_count, path_count) = grid.unwrap().determine_split_and_path_counts();

        assert_eq!(split_count, 21);
        assert_eq!(path_count, 40);
    }
}
