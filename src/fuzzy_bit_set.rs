use crate::fuzzy_bit::FBit;

use lazy_static::lazy_static;

use std::{
    collections::{HashSet},
    sync::{Mutex},
};

lazy_static!{
    static ref FUZZZY_BIT_SET: Mutex<HashSet<FBit>> = Mutex::new(HashSet::from_iter([
        FBit::TRUE,
        FBit::FALSE,
]));
}

pub fn deduplicate_fbit<'a>(fbit: &'a FBit) -> &'a FBit {
    let mut set = FUZZZY_BIT_SET.lock().unwrap();
    if let Some(original) = set.get(&fbit) {
        original
    } else {
        set.insert(*fbit);
        fbit
    }
}

pub fn get_set_size() -> usize {
    FUZZZY_BIT_SET.lock().unwrap().len()
}
