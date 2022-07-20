#!/usr/bin/env rust-script -o
//!
//! ```cargo
//! [dependencies]
//! term_macros = { git = "https://github.com/langwitch-tools/langwitch-preprocessing-toolkit" }
//! ```
use term_macros::*;

fn main() {
    tool! {
        args:
            - no_punctuation;
            - no_numbers;
            - lowercase;
        ;

        body: || {
            readin!(wtr, |line: &[u8]| {
                let line = std::str::from_utf8(line);
                if line.is_err() {
                    return;
                }
                let line = line.unwrap();
                let clean_line = line
                    .trim()
                    .chars()
                    .map(|c| {
                        if c.is_whitespace() || c.is_alphabetic() || c.is_numeric() && !no_numbers || c.is_ascii_punctuation() && !no_punctuation {
                            c
                        } else {
                            ' '
                        }
                    })
                    .fold(String::with_capacity(line.len()), |mut accum, c| {
                        if accum
                            .chars()
                            .last()
                            .map(|c1| c1.is_whitespace())
                            .unwrap_or_else(|| false)
                            && c.is_whitespace()
                        {
                        } else {
                            match lowercase {
                                true => accum.extend(c.to_lowercase()),
                                false => accum.push(c)
                            };
                        }
                        accum
                    });
                if clean_line.len() > 1 {
                    let _ = wtr.write_all(clean_line.as_bytes());
                    let _ = wtr.write_all(b"\n");
                }
            });
        }

    };
}