use bitvec::array::BitArray;
use bitvec::order::Lsb0;

type BoardBitset = BitArray<u64, Lsb0>;
pub struct Board {
    white_pawns: BoardBitset,
    white_knights: BoardBitset,
}

impl Default for Board {
    fn default() -> Self {
        Self {
            white_pawns: BitArray::new(0x0000_0000_0000_ff00u64),
            white_knights: BitArray::new(0x0000_0000_0000_0042u64)
        }
    }
}