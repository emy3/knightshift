use std::fmt;
use std::ops::{BitAnd, BitOr, BitXor, Not};

#[derive(PartialEq, Eq, PartialOrd, Clone, Copy, Debug, Default, Hash)]
pub struct BitBoard(pub u64);

impl BitBoard {
    pub const EMPTY: BitBoard = BitBoard(0);
    //pub const ALL: BitBoard = BitBoard(0xFFFFFFFFFFFFFFFF);

    // Display the BitBoard
    pub fn print(&self) {
        const LAST_BIT: u64 = 63;
        for rank in 0..8 {
            for file in (0..8).rev() {
                let mask = 1u64 << (LAST_BIT - (rank * 8) - file);
                let char = if self.0 & mask != 0 { '1' } else { '0' };
                print!("{char}");
            }
            println!();
        }
    }

    // Create the BitBoard
    pub fn from_square(square: u8) -> Self {
        debug_assert!(square < 64);
        BitBoard(1u64 << square)
    }

    // Check if square needs a piece is_set(square)
    pub fn is_set(&mut self, square: u8) -> bool {
        (self.0 & (1u64 << square)) != 0
    }

    // Add a piece set(square)
    pub fn set_bit(&mut self, square: u8) {
        self.0 |= 1u64 << square;
    }

    // Remove a piece clear(square)
    // Count pieces count(piece)
    // Find all pieces - iterate over?
}
