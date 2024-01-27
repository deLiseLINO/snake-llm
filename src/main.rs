mod board;
mod direction;
mod events;
mod point;
mod snake;
use board::Board;

fn main() {
    let mut board = Board::new(100, 50);

    board.start();
}
