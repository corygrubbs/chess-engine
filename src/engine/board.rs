use std::ops::BitOr;
use bitvec::array::BitArray;
use bitvec::order::Lsb0;

pub type BoardBitset = BitArray<u64, Lsb0>;
pub struct Board {
    pub(crate) white_pawns: BoardBitset,
    pub(crate) white_knights: BoardBitset,
    pub(crate) white_bishops: BoardBitset,
    pub(crate) white_rooks: BoardBitset,
    pub(crate) white_queens: BoardBitset,
    pub(crate) white_king: BoardBitset,

    pub(crate) black_pawns: BoardBitset,
    pub(crate) black_knights: BoardBitset,
    pub(crate) black_bishops: BoardBitset,
    pub(crate) black_rooks: BoardBitset,
    pub(crate) black_queens: BoardBitset,
    pub(crate) black_king: BoardBitset,

    pub(crate) white_board: BoardBitset,
    pub(crate) black_board: BoardBitset,
    
    pub(crate) complete_board: BoardBitset,
    pub(crate) empty_board: BoardBitset
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
        let starting_empty_board = !starting_complete_board;
        
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
            
            complete_board: starting_complete_board,
            empty_board: starting_empty_board
        }
    }
}