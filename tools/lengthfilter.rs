#!/usr/bin/env rust-script -o
//!
//! ```cargo
//! [dependencies]
//! term_macros = { git = "https://github.com/langwitch-tools/langwitch-preprocessing-toolkit" }
//! ```
use term_macros::*;
//use std::iter::FromIterator;
fn main() {

    tool! {
        args:
            - min_words: usize = 0;
            - max_words: usize = 10;
            - max_chars: usize = max_words * 15;
        ;

        body: || {
            filter_in!(|line: &[u8]| {
                let word_count = line.split(|c| c == &b' ').count();
                if line.len() > max_chars || word_count > max_words || word_count < min_words {
                    return false;
                } else {
                    return true;
                }
            });
        }

    };
}