//! Random utility functions that might be useful for thing
#![allow(dead_code)]

use std::hash::BuildHasherDefault;

use hashers::fx_hash::FxHasher;

pub type HashMap<K, V> = std::collections::HashMap<K, V, BuildHasherDefault<FxHasher>>;
pub type HashSet<T> = std::collections::HashSet<T, BuildHasherDefault<FxHasher>>;

/// Return mutable references to two different elements in a slice
pub fn get_2_mut<T>(slice: &mut [T], first: usize, second: usize) -> [&mut T; 2] {
    assert!(first < slice.len());
    assert!(second < slice.len());
    assert!(first != second);

    let slice = slice.as_mut_ptr();
    // SAFETY: first and second are within bounds, and do not refer to the same element
    unsafe {
        [
            slice.add(first).as_mut().unwrap(),
            slice.add(second).as_mut().unwrap(),
        ]
    }
}

/// Parse a u64 from an ascii string. Slightly faster than FromStr.
pub fn parse_u64(b: impl AsRef<[u8]>) -> Option<u64> {
    let b = b.as_ref();

    let mut out = 0;
    if b.iter().any(|b| !b.is_ascii_digit()) {
        return None;
    }

    for (i, &b) in b.iter().rev().enumerate() {
        let n = u64::from(b - b'0');
        out += n * (10u64.pow(i as u32));
    }
    Some(out)
}
