
use std::ops::{BitAnd, BitOr, BitXor, Not};
use std::hash::{Hash, Hasher};
use rand::thread_rng;
use rand::{Rng};
use seq_macro::seq;

type Block = u128;
const BLOCK_COUNT: usize = 64;
type BlockList = [Block; BLOCK_COUNT];

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct FBitHash {
    blocks: BlockList,
    hash: u64,
}

impl FBitHash {

    pub const FALSE: FBitHash = Self::from([Block::MIN; BLOCK_COUNT]);
    pub const TRUE: FBitHash  = Self::from([Block::MAX; BLOCK_COUNT]);

    pub fn new() -> FBitHash {
        let mut blocks: BlockList = [0; BLOCK_COUNT];

        for block in blocks.iter_mut() {
            *block = thread_rng().gen();
        }

        Self::from(blocks)
    }

    pub const fn from(blocks: BlockList) -> FBitHash {
        FBitHash {
            blocks,
            hash: {
                let mut hash: u64 = 0;

                assert!(BLOCK_COUNT == 64);
                seq!(i in 0..64 {
                    hash = hash.wrapping_add(((blocks[i] >> 00) as u64).wrapping_mul(3));
                    hash = hash.wrapping_add(((blocks[i] >> 64) as u64).wrapping_mul(7));
                    hash ^= hash << 13;
                    hash ^= hash >> 7;
                    hash ^= hash << 17;
                });

                hash
            }
        }
    }

    pub fn combine(lhs: &FBitHash, rhs: &FBitHash, func: impl Fn(Block, Block) -> Block) -> FBitHash {
        let mut blocks: BlockList = [0; BLOCK_COUNT];

        for (block, (l, r)) in blocks.iter_mut().zip(lhs.blocks.iter().zip(rhs.blocks.iter())) {
            *block = func(*l, *r);
        }

        Self::from(blocks)
    }
}

impl Hash for FBitHash {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hash.hash(state);
    }
}

impl BitAnd for FBitHash {
    type Output = FBitHash;

    fn bitand(self, rhs: FBitHash) -> FBitHash {
        Self::combine(&self, &rhs, |l, r| l & r)
    }
}

impl BitOr for FBitHash {
    type Output = FBitHash;

    fn bitor(self, rhs: FBitHash) -> FBitHash {
        Self::combine(&self, &rhs, |l, r| l | r)
    }
}

impl BitXor for FBitHash {
    type Output = FBitHash;

    fn bitxor(self, rhs: FBitHash) -> FBitHash {
        Self::combine(&self, &rhs, |l, r| l ^ r)
    }
}

impl Not for FBitHash {
    type Output = FBitHash;

    fn not(self) -> FBitHash {
        let mut blocks: BlockList =  [0; BLOCK_COUNT];

        for (i, block) in blocks.iter_mut().zip(self.blocks.iter()) {
            *i = !*block;
        }

        Self::from(blocks)
    }
}