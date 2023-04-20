use maze::Maze;

mod maze;

fn main() {
    let mut m = Maze::from_square_size(16);
    m.deep_first_search();

    m.display();
}
