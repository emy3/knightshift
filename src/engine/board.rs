use std::fmt;
use std::fmt::{Debug, Formatter};

#[derive(PartialEq, Eq, PartialOrd, Clone, Copy, Debug, Default, Hash)]
pub struct BitBoard(pub u64);

impl BitBoard {
    pub const EMPTY: BitBoard = BitBoard(0);
}

// Check if square needs a piece is_set(square)
// Add a piece set(square)
// Remove a piece clear(square)
// Count pieces count(piece)
// Find all pieces - iterate over?
