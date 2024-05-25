use crate::fuzzy_bit::FBit;

use std::fmt;
use std::ops::Index;
use std::ops::{Add, Mul, Neg, Shl, Shr, Sub};
use std::ops::{BitAnd, BitOr, BitXor, Not};

use array_init::array_init;

#[derive(Clone)]
pub struct FInt<const L: usize> {
    bits: [FBit; L],
}

pub type FInt8 = FInt<8>;
pub type FInt16 = FInt<16>;
pub type FInt32 = FInt<32>;
pub type FInt64 = FInt<64>;

impl<const L: usize> FInt<L> {
    pub fn init() -> FInt<L> {
        FInt {
            bits: [FBit::FALSE; L],
        }
    }

    pub fn from_slice(slice: [FBit; L]) -> FInt<L> {
        FInt { bits: slice }
    }

    pub fn build<'a>(mut builder: impl FnMut(usize) -> &'a FBit) -> &'a FInt<L> {
        let mut result = Self::init();

        for (i, bit) in result.bits.iter_mut().enumerate() {
            *bit = *builder(i);
        }

        &result
    }

    // This is some horrible type trickery to get a type that I can loop over the bits with
    pub fn from<'a, I: BitAnd<usize> + Copy>(n: I) -> &'a FInt<L>
    where
        I::Output: Into<usize>,
    {
        &FInt::build(|i| FBit::from(0 != (n & ((1 as usize) << i)).into()))
    }

    pub fn resize<const L2: usize>(self) -> FInt<L2> {
        *FInt::build(|i| &self[i])
    }

    pub fn collapse(&self) -> usize {
        let mut result: usize = 0;

        for i in 0..64 {
            if self[i].collapse() {
                result |= (1 as usize) << i;
            }
        }

        result
    }
}

impl<const L1: usize> FInt<L1> {
    pub fn combine<const L2: usize>(elements: &[&FInt<L1>; L2]) -> FInt<{ L1 * L2 }> {
        *FInt::<{ L1 * L2 }>::build(|i| &elements[i / L1][i % L1])
    }

    pub fn split<'a, const L2: usize>(self) -> [&'a FInt<L2>; (L1 + L2 - 1) / L2] {
        array_init(|e| FInt::build(|i| &self[L2 * e + i]))
    }
}

impl<const L: usize> Default for FInt<L> {
    fn default() -> Self {
        FInt::init()
    }
}

impl<const L: usize> Index<usize> for FInt<L> {
    type Output = FBit;

    fn index(&self, index: usize) -> &Self::Output {
        &self.bits.get(index).unwrap_or_default()
    }
}

impl<const L: usize> Neg for &FInt<L> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        !self + FInt::<L>::from(1)
    }
}

impl<const L: usize> Add for &FInt<L> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut carry = FBit::FALSE;
        FInt::<L>::build(|i| FBit::add_carry(&self[i], &rhs[i], &mut carry))
    }
}

impl<const L: usize> Sub for &FInt<L> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + -rhs
    }
}

impl<const L: usize> Shl<usize> for &FInt<L> {
    type Output = Self;

    fn shl(self, shift: usize) -> Self::Output {
        FInt::<L>::build(|i| &self[i.wrapping_sub(shift)])
    }
}

impl<const L: usize> Shr<usize> for &FInt<L> {
    type Output = Self;

    fn shr(self, shift: usize) -> Self::Output {
        FInt::<L>::build(|i| &self[i.wrapping_add(shift)])
    }
}

impl<const L: usize> Mul<&FBit> for &FInt<L> {
    type Output = Self;

    fn mul(self, rhs: &FBit) -> Self::Output {
        FInt::<L>::build(|i| &self[i] & rhs)
    }
}

impl<const L: usize> Mul for &FInt<L> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut result = FInt::<L>::init();

        for i in 0..L {
            result = *(&result + (self << i) * &rhs[i]);
        }

        &result
    }
}

impl<const L: usize> Not for &FInt<L> {
    type Output = Self;

    fn not(self) -> Self::Output {
        FInt::<L>::build(|i| !&self[i])
    }
}

impl<const L: usize> BitAnd for &FInt<L> {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        FInt::<L>::build(|i| &self[i] & &rhs[i])
    }
}

impl<const L: usize> BitOr for &FInt<L> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        FInt::<L>::build(|i| &self[i] | &rhs[i])
    }
}

impl<const L: usize> BitXor for &FInt<L> {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        FInt::<L>::build(|i| &self[i] ^ &rhs[i])
    }
}

impl<const L: usize> FInt<L> {
    pub fn lrotate(&self, shift: usize) -> &Self {
        let shift = shift.rem_euclid(L);
        (self << shift) | (self >> (L - shift))
    }

    pub fn rrotate(& self, shift: usize) -> &Self {
        let shift = shift.rem_euclid(L);
        (self >> shift) | (self << (L - shift))
    }
}

impl<const L: usize> fmt::Debug for FInt<L> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const CHUNK_SIZE: usize = 4;

        writeln!(f, "FInt{} [", L)?;

        for (i, chunk) in self.bits.chunks(CHUNK_SIZE).enumerate() {
            write!(
                f,
                "\t{}..{}: ",
                CHUNK_SIZE * i,
                CHUNK_SIZE * i + CHUNK_SIZE - 1
            )?;

            for bit in chunk {
                write!(f, "{:?}, ", bit)?;
            }

            writeln!(f)?;
        }

        write!(f, "]")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fint_arithmetic() {
        let a = FInt8::from(20);
        let b = FInt8::from(10);

        assert_eq!((a + b).collapse(), 30);
        assert_eq!((a - b).collapse(), 10);
        assert_eq!((a * b).collapse(), 200);

        let c = FInt32::from(2000);
        let d = FInt32::from(1000);

        assert_eq!((c + d).collapse(), 3000);
        assert_eq!((c - d).collapse(), 1000);
        assert_eq!((c * d).collapse(), 2000000);
    }

    #[test]
    fn test_fint_resize() {
        let a = FInt8::from(42);
        let b = a.resize::<32>();

        assert_eq!(b.collapse(), 42);

        let c = FInt32::from(123456);
        let d = c.resize::<8>();

        assert_eq!(d.collapse(), 123456 & 0xFF);
    }
}
