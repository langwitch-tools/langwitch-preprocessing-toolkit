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
use fnv::FnvHasher;
use std::hash::Hash;
use std::hash::Hasher;
use nohash_hasher::IntSet;

fn hash_str(s: &[u8]) -> u32 {
    let mut h = FnvHasher::with_key(0);
    s.hash(&mut h);
    h.finish() as u32
}

fn main() {
    let is_space = |c: &u8| c == &b' ';
    let mut set1 = IntSet::<u32>::default();
    let mut set2 = IntSet::<u32>::default();
    let into_set = |data: &[u8], set: &mut IntSet<u32>| data.split(is_space).map(|w| hash_str(w)).for_each(|w| {set.insert(w);});
    filter_in!(|line: &[u8]| {
        let mut split = line.split(|c| c == &b'\t');
        let part1 = split.next();
        let part2 = split.next();
        if part2.is_none() {
            return false;
        }
        into_set(part1.unwrap(), &mut set1);
        into_set(part2.unwrap(), &mut set2);
        let passed_check = set1.is_disjoint(&set2);
        set1.clear();
        set2.clear();
        passed_check
    });
}