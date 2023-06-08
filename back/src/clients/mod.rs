use std::io;
use serde_json::{Value, json};

use crate::maze::{Maze, cells::{Coord, Direction}};
use crate::api::Object;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Client {
    pub id: String,

    pub name: String,

    pub is_playing: bool,

    // Has a maze only if he is playing
    #[serde(skip_serializing)]
    pub maze: Option<Maze>,

    // Has a curr_cell only if he is playing
    // This reprensent the cell on which the player is
    pub curr_cell: Option<Coord>,
}

impl Client {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            is_playing: false,
            maze: None,
            curr_cell: None,
        }
    }

    pub fn start_game(&mut self) -> io::Result<()> {
        let mut m = Maze::from_square_size(10);
        m.deep_first_search();

        let center = m.get_center();

        self.maze = Some(m);
        self.curr_cell = center;
        self.is_playing = true;

        Ok(())
    }

    pub fn check_win(&self, dir: Direction) -> bool {
        let maze = self.maze.as_ref().unwrap();
        let coord = self.curr_cell.as_ref().unwrap();

        let cell = maze.get_cell(Coord { x: coord.x, y: coord.y }).unwrap();

        maze.is_going_out(cell, &dir)
    }

    pub fn move_cell(&mut self, dir: Direction) -> bool {
        let cell = self.curr_cell.as_ref().unwrap();
        let cell = self.maze.as_ref().unwrap().get_cell(Coord { x: cell.x, y: cell.y }).unwrap();

        match cell.reach(dir) {
            Some(coord) => {
                self.curr_cell = Some(coord);
                true
            }
            None => false,
        }
    }
}

impl Object for Client {
    fn to_map(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();

        map.insert("id".to_string(), json!(self.id.clone()));
        map.insert("name".to_string(), json!(self.name.clone()));
        map.insert("is_playing".to_string(), json!(self.is_playing.to_string()));

        if self.curr_cell.is_some() {
            let maze = self.maze.as_ref().unwrap();
            let coord = self.curr_cell.as_ref().unwrap();

            let cell = maze.get_cell(Coord { x: coord.x, y: coord.y }).unwrap();

            map.insert("curr_cell".to_string(), json!(cell.walls));
            map.insert("pos".to_owned(), json!({
                "x": coord.x,
                "y": coord.y,
            }));
        }

        map
    }
}
