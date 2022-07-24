#!/usr/bin/env rust-script -o
//!
//! ```cargo
//! [dependencies]
//! term_macros = { git = "https://github.com/langwitch-tools/langwitch-preprocessing-toolkit" }
//! ```

use term_macros::*;

fn no_punctuation(w: &[u8]) -> Vec<u8> {
    w.iter().copied().filter(|c| !c.is_ascii_punctuation()).map(|c| if c.is_ascii_alphabetic() { c.to_ascii_lowercase() } else { c }).collect()
}

fn main() {
    let mut map = std::collections::HashMap::with_capacity(1000000);
    readin!(_tx, |s: &[u8]| {
        s.split(|c| c.is_ascii_whitespace() || c.is_ascii_digit()).filter(|w| w.len() > 0).for_each(|w| {
            let key = no_punctuation(w);
            if !(key.len() > 0) {
                return;
            }
            if let Some(entry) = map.get_mut(&key) {
                *entry += 1;
            } else {
                map.insert(key, 1);
            }
        })
    });
    let mut freqs: Vec<_> = map.into_iter().collect();
    freqs.sort_by_key(|(_k, v)| *v as i64);

    freqs.into_iter().rev().for_each(|(k, _v)| {
        let _ = std::str::from_utf8(k.as_slice()).map(|s: &str| {
            println!("{}", s);
        });
    });
}