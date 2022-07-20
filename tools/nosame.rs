#!/usr/bin/env rust-script -o
//!
//! ```cargo
//! [dependencies]
//! term_macros = { git = "https://github.com/langwitch-tools/langwitch-preprocessing-toolkit" }
//! fnv = "1.0.7"
//! nohash-hasher = "0.2.0"
//! ```

//use rayon::prelude::*;
use term_macros::*;
use std::iter::FromIterator;
use fnv::FnvHasher;
use std::hash::Hash;
use std::hash::Hasher;
use nohash_hasher::IntSet;

fn hash_str(s: &[u8]) -> u64 {
    let mut h = FnvHasher::with_key(0);
    s.hash(&mut h);
    h.finish()
}
fn no_punctuation(w: &str) -> String {
    w.chars().filter(|c| c.is_alphabetic() || c.is_whitespace()).collect::<String>().to_lowercase()
}

fn words(w: &str) -> IntSet<u64> {
    IntSet::from_iter(no_punctuation(w).split(" ").map(|s| hash_str(s.as_bytes())))
}

fn main() {
    filter_in!(|line: &[u8]| {
        let line = std::str::from_utf8(line);
        if line.is_err() {
            return false;
        }
        let line = line.unwrap();
        let mut parts = line.split("\t");
        let p1 = parts.next().map(words);
        let p2 = parts.next().map(words);
        if p1.is_none() || p2.is_none() {
            return false;
        };
        if p1.unwrap().intersection(&p2.unwrap()).count() > 0 {
            return false;
        }
        return true;
    });
}