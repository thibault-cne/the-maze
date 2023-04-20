use maze::Maze;

mod maze;

fn main() {
    let mut m = Maze::from_square_size(16);
    m.deep_first_search();

    m.display();

    let c = m.cells[m.cells.len() / 2].clone();

    println!("x: {} y: {} way: {}", c.coord.x, c.coord.y, m.has_way_out(&c));
}
