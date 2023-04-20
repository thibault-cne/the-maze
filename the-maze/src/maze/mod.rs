use std::collections::HashMap;
use rand::seq::SliceRandom;

use crate::maze::cells::Cell;

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
                    cell1.remove_wall(cell2);

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

        self.add_way_out();
    }

    pub fn add_way_out(&mut self) {
        let mut ways = Vec::new();

        for i in 0..self.w_size {
            let c1 = Cell::from_coord(i as isize, 0);
            let c2 = Cell::from_coord(i as isize, self.h_size as isize - 1);

            ways.push(c1);
            ways.push(c2);
        }

        for i in 0..self.h_size {
            let c1 = Cell::from_coord(0, i as isize);
            let c2 = Cell::from_coord(self.w_size as isize - 1, i as isize);

            ways.push(c1);
            ways.push(c2);
        }

        let way = ways.choose(&mut rand::thread_rng()).unwrap();

        let cells = self.cells.clone();
        let mut indexes = cells
            .iter()
            .enumerate()
            .filter(|(_, c)| c.coord.x == way.coord.x && c.coord.y == way.coord.y)
            .map(|(i, _)| i);

        while let Some(i) = indexes.next() {
            if way.coord.x == 0 {
                self.cells[i].walls[0] = false;
            }

            if way.coord.x == self.w_size as isize - 1 {
                self.cells[i].walls[2] = false;
            }

            if way.coord.y == 0 {
                self.cells[i].walls[3] = false;
            }

            if way.coord.y == self.h_size as isize - 1 {
                self.cells[i].walls[1] = false;
            }
        }
            
    }

    pub fn has_way_out(&mut self, cell: &Cell) -> bool {
        // Reset visited cells
        self.cells
            .iter_mut()
            .for_each(|c| c.visited = false);

        let mut queue: Vec<&Cell> = Vec::new();
        let mut map: HashMap<(isize, isize), bool> = std::collections::HashMap::new();

        queue.push(cell);

        while let Some(cell) = queue.pop() {
            map.insert((cell.coord.x, cell.coord.y), true);

            if self.is_way_out(cell) {
                return true;
            }

            let neighbors = cell.find_neighbors(&self.cells);
            
            for &neighbor in neighbors.iter() {
                if cell.can_reach(&self.cells[neighbor]) && !map.contains_key(&(self.cells[neighbor].coord.x, self.cells[neighbor].coord.y)) {
                    queue.push(&self.cells[neighbor]);
                }
            }
        }

        false
    }

    pub fn is_way_out(&self, cell: &Cell) -> bool {
        if cell.coord.y == 0 && !cell.walls[3] {
            return true;
        }

        if cell.coord.y as usize == self.w_size - 1 && !cell.walls[1] {
            return true;
        }

        if cell.coord.x == 0 && !cell.walls[0] {
            return true;
        }

        if cell.coord.x as usize == self.h_size - 1 && !cell.walls[2] {
            return true;
        }

        false
    }

    pub fn display(&self) {
        for i in 0..self.h_size {
            self.display_row(i);
        }
    }

    fn display_row(&self, row: usize) {
        let mut upper_buffer = String::new();
        let mut mid_buffer = String::new();
        let mut lower_buffer = String::new();

        let mut cells = self.cells
            .iter()
            .filter(|c| c.coord.x == row as isize);

        while let Some(cell) = cells.next() {
            upper_buffer.push_str("+");
            lower_buffer.push_str("+");

            if cell.walls[0] && cell.coord.x == 0 {
                upper_buffer.push_str("---");
            } else if cell.coord.x == 0 {
                upper_buffer.push_str("   ");
            }

            if cell.walls[2] {
                lower_buffer.push_str("---");
            } else {
                lower_buffer.push_str("   ");
            }
            
            if cell.walls[3] && cell.coord.y == 0 {
                mid_buffer.push_str("|");
            } else if cell.coord.y == 0 {
                mid_buffer.push_str(" ");
            }

            mid_buffer.push_str("   ");

            if cell.walls[1] {
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