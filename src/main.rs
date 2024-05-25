#![feature(const_trait_impl)]
#![feature(generic_const_exprs)]

pub mod fuzzy_bit_hash;
pub mod fuzzy_bit_set;
pub mod fuzzy_bit;
pub mod fuzzy_int;
pub mod fuzzy_sha1;
pub mod fuzzy_sha256;

use fuzzy_int::*;

use crate::fuzzy_bit::FBit;

use fuzzy_sha1::*;
use fuzzy_sha256::*;

fn main() {
    let tap = 0.5;

    let base = [
        FInt8::from('t' as usize),
        FInt8::from('e' as usize),
        FInt8::from('s' as usize),
        FInt8::from('t' as usize),
        FInt8::build(|_| FBit::from(false)),
    ];

    let tapped = [
        FInt8::from('t' as usize),
        FInt8::from('e' as usize),
        FInt8::from('s' as usize),
        FInt8::from('t' as usize),
        FInt8::build(|i| {
            if i == 0 { FBit::from_float(tap) } else { FBit::from(false) }
        }),
    ];

    println!("");
    println!("");
    println!("SHA1 - BASE");
    println!("{:?}", fuzzy_sha1(&base));

    dbg!(fuzzy_bit_set::get_set_size());

    println!("");
    println!("SHA1 - TAPPED");
    println!("{:?}", fuzzy_sha1(&tapped));

    dbg!(fuzzy_bit_set::get_set_size());

    println!("");
    println!("");
    println!("SHA256 - BASE");
    println!("{:?}", fuzzy_sha256(&base));

    dbg!(fuzzy_bit_set::get_set_size());

    println!("");
    println!("SHA256 - TAPPED");
    println!("{:?}", fuzzy_sha256(&tapped));

    dbg!(FBit::TRUE);
    let a = FBit::from_float(0.5);
    let b = a ^ FBit::from_float(0.5);
    let c = b ^ a;
    dbg!(a);
    dbg!(b);
    dbg!(c);

}
