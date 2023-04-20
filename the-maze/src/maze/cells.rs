use std::fmt::Display;

// Coordinates of a grid cell
#[derive(Clone, Debug)]
pub struct Coord {
    pub x: isize,
    pub y: isize
}

// A cell of the grid
#[derive(Clone, Debug)]
pub struct Cell {
    // List of walls
    pub walls: [bool; 4],

    // Coordinates
    pub coord: Coord,

    // For generation purpose
    pub visited: bool,
}

// Directions
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Direction {
    Up,
    Left,
    Right,
    Down
}

impl Cell {
    pub fn from_coord(x: isize, y: isize) -> Self {
        Cell {
            walls: [true; 4],
            coord: Coord { x, y },
            visited: false,
        }
    }

    pub fn remove_wall(&mut self, cell: &mut Self) {
        let wall = self.cell_direction(cell);

        match wall {
            Direction::Up => {
                self.walls[0] = false;
                cell.walls[2] = false;
            },
            Direction::Right => {
                self.walls[1] = false;
                cell.walls[3] = false;
            },
            Direction::Down => {
                self.walls[2] = false;
                cell.walls[0] = false;
            },
            Direction::Left => {
                self.walls[3] = false;
                cell.walls[1] = false;
            },
        }
    }

    pub fn cell_direction(&self, cell: &Self) -> Direction {
        match self.coord.x  - cell.coord.x {
            x if x == 0 => {
                match self.coord.y - cell.coord.y {
                    y if y > 0 => Direction::Left,
                    y if y < 0 => Direction::Right,
                    _ => unreachable!()
                }
            }
            x if x > 0 => Direction::Up,
            x if x < 0 => Direction::Down,
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

    pub fn can_reach(&self, cell: &Self) -> bool {
        let dir = self.cell_direction(cell);

        match dir {
            Direction::Down => {
                if !self.walls[2] && !cell.walls[0] {
                    return  true;
                }
            },
            Direction::Up => {
                if !self.walls[0] && !cell.walls[2] {
                    return  true;
                } 
            },
            Direction::Left => {
                if !self.walls[3] && !cell.walls[1] {
                    return  true;
                }
            },
            Direction::Right => {
                if !self.walls[1] && !cell.walls[3] {
                    return  true;
                }
            },
        }

        false
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x: {} y: {}", self.coord.x, self.coord.y)?;

        Ok(())
    }
}