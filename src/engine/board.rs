use std::ops::BitOr;
use bitvec::array::BitArray;
use bitvec::order::Lsb0;

type BoardBitset = BitArray<u64, Lsb0>;
pub struct Board {
    white_pawns: BoardBitset,
    white_knights: BoardBitset,
    white_bishops: BoardBitset,
    white_rooks: BoardBitset,
    white_queens: BoardBitset,
    white_king: BoardBitset,

    black_pawns: BoardBitset,
    black_knights: BoardBitset,
    black_bishops: BoardBitset,
    black_rooks: BoardBitset,
    black_queens: BoardBitset,
    black_king: BoardBitset,

    white_board: BoardBitset,
    black_board: BoardBitset,
    complete_board: BoardBitset
}

impl Default for Board {
    fn default() -> Self {
        let starting_white_pawns = BitArray::new(0x0000_0000_0000_ff00u64);
        let starting_white_knights = BitArray::new(0x0000_0000_0000_0042u64);
        let starting_white_bishops = BitArray::new(0x0000_0000_0000_0024u64);
        let starting_white_rooks = BitArray::new(0x0000_0000_0000_0081u64);
        let starting_white_queens = BitArray::new(0x0000_0000_0000_0008u64);
        let starting_white_king = BitArray::new(0x0000_0000_0000_0010);

        let starting_black_pawns = BitArray::new(0x00ff_0000_0000_0000u64);
        let starting_black_knights = BitArray::new(0x4200_0000_0000_0000u64);
        let starting_black_bishops = BitArray::new(0x2400_0000_0000_0000u64);
        let starting_black_rooks = BitArray::new(0x8100_0000_0000_0000u64);
        let starting_black_queens = BitArray::new(0x0800_0000_0000_0000u64);
        let starting_black_king = BitArray::new(0x1000_0000_0000_0000);

        let starting_white_board = starting_white_pawns | starting_white_knights | starting_white_bishops | starting_white_rooks | starting_white_queens | starting_white_king;
        let starting_black_board = starting_black_pawns | starting_black_knights | starting_black_bishops | starting_black_rooks | starting_black_queens | starting_black_king;
        let starting_complete_board = starting_white_board | starting_black_board;


        let complete_board = starting_white_pawns | starting_black_pawns;
        Self {
            white_pawns: starting_white_pawns,
            white_knights: starting_white_knights,
            white_bishops: starting_white_bishops,
            white_rooks: starting_white_rooks,
            white_queens: starting_white_queens,
            white_king: starting_white_king,

            black_pawns: starting_black_pawns,
            black_knights: starting_black_knights,
            black_bishops: starting_black_bishops,
            black_rooks: starting_black_rooks,
            black_queens: starting_black_queens,
            black_king: starting_black_king,

            white_board: starting_white_board,
            black_board: starting_black_board,
            complete_board: starting_complete_board
        }
    }
}