mod engine;

fn main() {
    use engine::board::BitBoard;

    // Create a bitboard with some pieces
    let board = BitBoard(0xFF00);
    board.print();
}
