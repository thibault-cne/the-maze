// Coordinates of a grid cell
#[derive(Clone)]
pub struct Coord {
    pub x: isize,
    pub y: isize
}

// A cell of the grid
#[derive(Clone)]
pub struct Cell {
    // List of walls
    pub walls: Vec<Direction>,

    // Coordinates
    pub coord: Coord,

    // For generation purpose
    pub visited: bool,
}

// Directions
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Up,
    Left,
    Right,
    Down
}

impl Cell {
    pub fn from_coord(x: isize, y: isize) -> Self {
        Cell {
            walls: vec![Direction::Up, Direction::Down, Direction::Left, Direction::Right],
            coord: Coord { x, y },
            visited: false,
        }
    }

    pub fn remove_wall(&mut self, wall: Direction) {
        let new_walls = self.walls
            .drain(..)
            .filter(|&w| w != wall)
            .collect::<Vec<Direction>>();
        
        self.walls = new_walls;
    }

    pub fn cell_direction(&self, cell: &Self) -> Direction {
        match self.coord.x  - cell.coord.x {
            x if x == 0 => {
                match self.coord.y - cell.coord.y {
                    y if y > 0 => Direction::Up,
                    y if y < 0 => Direction::Down,
                    _ => unreachable!()
                }
            }
            x if x > 0 => Direction::Left,
            x if x < 0 => Direction::Right,
            _ => unreachable!()
        }
    }

    pub fn find_neighbors(&self, cells: &Vec<Cell>) -> Vec<usize> {
        cells
            .into_iter()
            .enumerate()
            .filter(|(_, c)| self.is_neighbors(c) && !c.visited)
            .map(|(i, _)| i)
            .collect::<Vec<usize>>()
    }

    pub fn is_neighbors(&self, cell: &Self) -> bool {
        match self.coord.x - cell.coord.x {
            x if x == 1 || x == -1 => {
                match self.coord.y - cell.coord.y {
                    y if y == 0 => true,
                    _ => false
                }
            },
            x if x == 0 => {
                match self.coord.y - cell.coord.y {
                    y if y == 1 || y == -1 => true,
                    _ => false
                } 
            }
            _ => false
        }
    }

    pub fn common_walls(&self, cells: &Vec<&Cell>) -> Vec<Direction> {
        let mut walls = Vec::new();

        for cell in cells.iter() {
            for wall in cell.walls.iter() {
                if !walls.contains(wall) {
                    walls.push(*wall);
                }
            }
        }

        for wall in self.walls.iter() {
            if !walls.contains(wall) {
                walls.push(*wall);
            }
        }

        walls
    }
}