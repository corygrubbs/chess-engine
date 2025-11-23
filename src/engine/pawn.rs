use std::ops::BitOr;
use crate::engine::board::{Board, BoardBitset};

const WHITE_PAWN_START_RANK: u64 = 0x0000_0000_0000_ff00u64;
const BLACK_PAWN_START_RANK: u64 = 0x00ff_0000_0000_0000u64;

impl Board {

    pub fn move_white_pawns(&mut self) -> BoardBitset {
        self.white_pawn_push().bitor(self.white_pawn_double_push())
    }

    pub fn move_black_pawns(&mut self) -> BoardBitset {
        self.black_pawn_push().bitor(self.black_pawn_double_push())
    }

    fn white_pawn_push(&mut self) -> BoardBitset {
        let mut pushes = self.white_pawns;
        pushes.shift_left(8);
        pushes & self.empty_board
    }

    fn black_pawn_push(&mut self) -> BoardBitset {
        let mut pushes = self.black_pawns;
        pushes.shift_right(8);
        pushes & self.empty_board
    }

    fn white_pawn_double_push(&mut self) -> BoardBitset {
        let start_rank_mask = BoardBitset::new(WHITE_PAWN_START_RANK);
        let mut pushes = self.white_pawns;
        pushes &= start_rank_mask;
        pushes.shift_left(16);
        pushes & self.empty_board
    }

    fn black_pawn_double_push(&mut self) -> BoardBitset {
        let start_rank_mask = BoardBitset::new(BLACK_PAWN_START_RANK);
        let mut pushes = self.black_pawns;
        pushes &= start_rank_mask;
        pushes.shift_right(16);
        pushes & self.empty_board
    }
}