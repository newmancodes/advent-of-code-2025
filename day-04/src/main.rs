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
    let mut room_width = 0u32;
    let mut room_height = 0;
    let mut roll_indexes = Vec::new();

    for (line_index, line) in reader.lines().enumerate() {
        // Increment room height as we've found another line.
        room_height += 1;

        match line {
            Ok(l) => {
                // Strip BOM if present
                let clean = l.trim_start_matches('\u{feff}');

                for (cell_index, char) in clean.chars().enumerate() {
                    if cell_index > room_width as usize {
                        room_width = cell_index as u32;
                    }

                    if char == '@' {
                        roll_indexes.push((cell_index as u32, line_index as u32));
                    }
                }
            },
            Err(_) => continue,
        }
    }

    let mut room_builder = RoomBuilder::new(room_width + 1, room_height);
    for roll_index in roll_indexes {
        room_builder = room_builder.with_roll_at(roll_index.0, roll_index.1);
    }
    let room = room_builder.build();

    println!("Room has {:?} accessible rolls.", room.count_accessible_rolls());
    let (_, removed_rolls) = Room::remove_accessible_rolls(room);
    println!("Removed {:?} rolls.", removed_rolls);
}

#[derive(Clone)]
enum Cell {
    Empty,
    Roll
}

struct Room {
    cells: Vec<Cell>,
    width: u32,
}

impl Room {
    fn count_accessible_rolls(&self) -> u32 {
        let mut accessible_rolls = 0;

        for (index, cell) in self.cells.iter().enumerate() {
            match cell {
                Cell::Roll if self.is_accessible(index) => accessible_rolls += 1,
                _ => continue,
            }
        }

        accessible_rolls
    }

    fn is_accessible(&self, index: usize) -> bool {
        let index = index as u32;
        let x = index % self.width;
        let y = index / self.width;
        let height = self.cells.len() as u32 / self.width;

        let has_left_neighbour = x > 0;
        let has_right_neighbour = x < self.width - 1;
        let has_top_neighbour = y > 0;
        let has_bottom_neighbour = y < height - 1;

        let mut neighbour_indexes = Vec::new();

        if has_top_neighbour {
            if has_left_neighbour {
                neighbour_indexes.push(index - 1 - self.width);
            }
            neighbour_indexes.push(index - self.width);
            if has_right_neighbour {
                neighbour_indexes.push(index + 1 - self.width);
            }
        }

        if has_left_neighbour {
            neighbour_indexes.push(index - 1);
        }
        if has_right_neighbour {
            neighbour_indexes.push(index + 1);
        }

        if has_bottom_neighbour {
            if has_left_neighbour {
                neighbour_indexes.push(index - 1 + self.width);
            }
            neighbour_indexes.push(index + self.width);
            if has_right_neighbour {
                neighbour_indexes.push(index + 1 + self.width);
            }
        }

        let mut adjacent_roll_count = 0;
        for neighbour_index in neighbour_indexes {
            match self.cells[neighbour_index as usize] {
                Cell::Roll => adjacent_roll_count += 1,
                _ => continue,
            }
        }

        adjacent_roll_count < 4
    }

    fn remove_accessible_rolls(mut room: Self) -> (Self, u32) {
        let mut total_accessible_rolls = 0;

        loop {
            let mut accessible_roll_indexes = Vec::new();

            for (index, cell) in room.cells.iter().enumerate() {
                match cell {
                    Cell::Roll if room.is_accessible(index) => accessible_roll_indexes.push(index),
                    _ => continue,
                }
            }

            let mut new_cells = room.cells.clone();
            for index in accessible_roll_indexes.iter() {
                new_cells[*index] = Cell::Empty;
            }
            room = Room { cells: new_cells, width: room.width };

            total_accessible_rolls += accessible_roll_indexes.len();
            if accessible_roll_indexes.len() == 0 {
                break;
            }
        }

        (room, total_accessible_rolls as u32)
    }
}

struct RoomBuilder {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl RoomBuilder {
    fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            cells: vec![Cell::Empty; width as usize * height as usize],
        }
    }

    fn with_roll_at(mut self, x: u32, y: u32) -> Self {
        if x < self.width && y < self.height {
            self.cells[(x + y * self.width) as usize] = Cell::Roll;
        }

        self
    }

    fn build(self) -> Room {
        Room {
            cells: self.cells,
            width: self.width,
        }
    }
}
#[cfg(test)]
mod test{
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::top_left(0, 0)]
    #[case::top_middle(0, 1)]
    #[case::top_right(0, 2)]
    #[case::centre_left(1, 0)]
    #[case::centre_middle(1, 1)]
    #[case::centre_right(1, 2)]
    #[case::bottom_left(2, 0)]
    #[case::bottom_middle(2, 1)]
    #[case::bottom_right(2, 2)]
    fn smallest_room_with_single_roll(#[case] x: u32, #[case] y: u32) {
        let room = RoomBuilder::new(3, 3)
            .with_roll_at(x, y)
            .build();

        assert_eq!(1, room.count_accessible_rolls());
    }

    #[test]
    fn smallest_room_with_corner_rolls() {
        let room = RoomBuilder::new(3, 3)
            .with_roll_at(0,0)
            .with_roll_at(2,0)
            .with_roll_at(2,2)
            .with_roll_at(0,2)
            .build();

        assert_eq!(4, room.count_accessible_rolls());
    }

    #[test]
    fn smallest_room_with_too_many_adjacent_rolls() {
        let room = RoomBuilder::new(3, 3)
            .with_roll_at(1,0)
            .with_roll_at(0, 1)
            .with_roll_at(1, 1)
            .with_roll_at(2, 1)
            .with_roll_at(1,2)
            .build();

        assert_eq!(4, room.count_accessible_rolls());
    }
}