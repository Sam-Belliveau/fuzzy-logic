use std::fmt;
use std::ops::{BitAnd, BitOr, BitXor, Not};
use std::hash::{Hash, Hasher};

use std::default::Default;
use std::sync::Arc;

use crate::fuzzy_bit_set::deduplicate_fbit;
use crate::fuzzy_bit_hash::FBitHash;
use crate::fuzzy_int::FInt;

pub type RepT = f64;

#[derive(Clone)]
pub struct FBit {
    p: RepT,
    hash: Arc<FBitHash>,
}

impl FBit {
    const TRUE_VALUE: RepT = 1.0;
    const FALSE_VALUE: RepT = 0.0;

    pub const TRUE: FBit = FBit::from_rep(Self::TRUE_VALUE, FBitHash::TRUE);
    pub const FALSE: FBit = FBit::from_rep(Self::FALSE_VALUE, FBitHash::FALSE);
}

impl FBit {
    const fn from_rep(p: RepT, hash: FBitHash) -> FBit {
        FBit {
            p,
            hash: Arc::new(hash),
        }
    }

    pub fn from_float<'a>(p: f64) -> &'a FBit {
        &FBit::from_rep(p, FBitHash::new())
    }

    pub const fn from(b: bool) -> &'static FBit {
        if b {
            &Self::TRUE
        } else {
            &Self::FALSE
        }
    }

    pub fn collapse(&self) -> bool {
        0.5 < self.p
    }
}

impl PartialEq for FBit {
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
    }
}

impl Eq for FBit {}

impl Hash for FBit {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hash.hash(state);
    }
}

impl Default for FBit {
    fn default() -> Self {
        Self::FALSE
    }
}

impl Default for &FBit {
    fn default() -> Self {
        &FBit::FALSE
    }
}

impl Not for &FBit {
    type Output = Self;

    fn not(self) -> Self::Output {
        deduplicate_fbit(&FBit {
            p: FBit::TRUE_VALUE - self.p,
            hash: Arc::from(! *self.hash),
        })
    }
}

impl BitAnd for &FBit {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        deduplicate_fbit(&FBit {
            p: self.p * rhs.p,
            hash: Arc::from(*self.hash & *rhs.hash),
        })
    }
}

impl BitOr for &FBit {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        deduplicate_fbit(&FBit {
            p: self.p + rhs.p - 1.0 * (self.p * rhs.p),
            hash: Arc::from(*self.hash | *rhs.hash),
        })
    }
}

impl BitXor for &FBit {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        deduplicate_fbit(&FBit {
            p: self.p + rhs.p - 2.0 * (self.p * rhs.p),
            hash: Arc::from(*self.hash | *rhs.hash),
        })
    }
}

impl FBit {
    pub fn piecewise<'a>(&'a self, true_value: &'a FBit, false_value: &'a FBit) -> &'a FBit {
        deduplicate_fbit(&FBit {
            p: (self.p) * true_value.p + (1.0 - self.p) * false_value.p,
            hash: Arc::from((*self.hash & *true_value.hash) | ((! *self.hash) & *false_value.hash)),
        })
    }

    pub fn piecewise_int<'a, const L: usize>(
        &'a self,
        true_value: &'a FInt<L>,
        false_value: &'a FInt<L>,
    ) -> &'a FInt<L> {
        &FInt::build(|i| self.piecewise(&true_value[i], &false_value[i]))
    }
}

impl FBit {
    pub fn add_carry<'a>(a: &'a Self, b: &'a Self, c: &'a mut Self) -> &'a FBit {
        let result = a ^ b ^ c;
        *c = *c.piecewise(a | b, a & b);
        result
    }
}

impl fmt::Debug for FBit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({})", self.p)
    }
}
