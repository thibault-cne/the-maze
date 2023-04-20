use rand::seq::SliceRandom;

use crate::maze::cells::{Cell, Direction};

mod cells;

pub struct Maze {
    pub w_size: usize,
    pub h_size: usize,


    pub cells: Vec<Cell>
}

impl Maze {
    pub fn from_square_size(size: usize) -> Self {
        let mut cells = Vec::new();

        // Fill the maze cells
        for i in 0..size {
            for j in 0..size {
                cells.push(Cell::from_coord(i as isize, j as isize))
            }
        }

        Maze {
            w_size: size,
            h_size: size,
            cells
        }
    }

    pub fn deep_first_search(&mut self) {
        let mut current: usize = 0;
        let mut backtrace: Vec<usize> = Vec::new();

        loop {
            self.cells[current].visited = true;
            let indexes = self.cells[current].find_neighbors(&self.cells);

            match indexes.choose(&mut rand::thread_rng()) {
                Some(&i) => {
                    self.cells[i].visited = true;
                    backtrace.push(current);

                    let (lower_part, higher_part) =
                        self.cells.split_at_mut(std::cmp::max(current, i));
                    let cell1 = &mut lower_part[std::cmp::min(current, i)];
                    let cell2 = &mut higher_part[0];
                    let dir = cell1.cell_direction(cell2);
                    cell1.remove_wall(dir);

                    current = i
                },
                None => {
                    if !backtrace.is_empty() {
                        current = backtrace[0];
                        backtrace.remove(0);
                    } else {
                        break;
                    }
                }
            }
        }
    }

    pub fn display(&self) {
        for i in 0..self.h_size {
            self.display_row(i);
        }
    }

    pub fn display_row(&self, row: usize) {
        let mut upper_buffer = String::new();
        let mut mid_buffer = String::new();
        let mut lower_buffer = String::new();

        let mut cells = self.cells
            .iter()
            .filter(|c| c.coord.x == row as isize);

        while let Some(cell) = cells.next() {
            upper_buffer.push_str("+");
            lower_buffer.push_str("+");

            let neighbors = cell
                .find_neighbors(&self.cells)
                .iter()
                .map(|&i| &self.cells[i])
                .collect::<Vec<&Cell>>();

            let walls = cell.common_walls(&neighbors);

            if walls.contains(&Direction::Up) && cell.coord.x == 0 {
                upper_buffer.push_str("---");
            } else if cell.coord.x == 0 {
                upper_buffer.push_str("   ");
            }

            if walls.contains(&Direction::Down) {
                lower_buffer.push_str("---");
            } else {
                lower_buffer.push_str("   ");
            }
            
            if walls.contains(&Direction::Left) && cell.coord.y == 0 {
                mid_buffer.push_str("|");
            } else if cell.coord.y == 0 {
                mid_buffer.push_str(" ");
            }

            mid_buffer.push_str("   ");

            if walls.contains(&Direction::Right) {
                mid_buffer.push_str("|");
            } else {
                mid_buffer.push_str(" ");
            }
        }

        if row == 0 {
            println!("{upper_buffer}");
        }
    
        println!("{mid_buffer}");
        println!("{lower_buffer}");
    }
}