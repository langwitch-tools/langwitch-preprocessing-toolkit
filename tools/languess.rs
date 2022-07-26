#!/usr/bin/env rust-script -o
//!
//! ```cargo
//! [dependencies]
//! lz4_flex = { version = "0.9.0", default-features = false }
//! rayon = "1.5.3"
//! memmap = "0.7.0"
//! once_cell = "1.12.0"
//! ```

use lz4_flex::compress_prepend_size;
use memmap::MmapOptions;
use once_cell::sync::Lazy;
use rayon::prelude::*;
use std::collections::HashMap;

use std::io::{Error, ErrorKind};

fn compression_ratio(
    f1: &[u8],
    f2: &[u8],
    compr_f1_len: usize,
    compr_f2_len: usize,
) -> Result<f64, Error> {
    if f1.len() + f2.len() == 0 {
        return Err(Error::new(ErrorKind::InvalidData, "File is empty"));
    }
    let compressed_together =
        compress_prepend_size(&f1.iter().chain(f2.iter()).copied().collect::<Vec<u8>>());

    let actual = (compressed_together.len()) as f64 / (compr_f1_len + compr_f2_len) as f64;
    Ok(actual)
}

/// Returns a list of all files found in the top level of a directory (and not within any subdirectories). Ignores folders.
fn get_files(path: &str) -> Result<Vec<std::path::PathBuf>, std::io::Error> {
    let files = std::fs::read_dir(path)?
        .into_iter()
        .map(|entry| {
            entry.and_then(|e| {
                if e.path().is_file() {
                    Ok(e.path().to_path_buf())
                } else {
                    Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "Not a file",
                    ))
                }
            })
        })
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap())
        .collect::<Vec<_>>();
    Ok(files)
}

static LANGUAGE_DIR: Lazy<String> = Lazy::new(|| {
    // Get the first arg
    std::env::args().nth(1).expect("Please provide a path")
});

static LENGTHS: Lazy<std::collections::HashMap<String, (memmap::Mmap, usize)>> = Lazy::new(|| {
    let files = get_files(&LANGUAGE_DIR)
        .unwrap()
        .into_iter()
        .map(|fname| (fname.clone(), std::fs::File::open(fname).unwrap()))
        .collect::<Vec<_>>();
    files
        .into_par_iter()
        .map(|(fname, file)| {
            // Safety: no
            let mmap = unsafe { MmapOptions::new().map(&file).unwrap() };
            let byte_length = compress_prepend_size(&mmap[..]).len();
            (fname.to_string_lossy().to_string(), (mmap, byte_length))
        })
        .collect()
});

fn main() {
    let top_n = std::env::args()
        .nth(2)
        .expect("Please provide the maximum no. of languages to display")
        .parse::<usize>()
        .unwrap();
    let average_length = LENGTHS
        .values()
        .map(|(_, byte_length)| byte_length)
        .sum::<usize>() as f64
        / LENGTHS.len() as f64;
    let mut input_str = std::env::args()
        .nth(3)
        .expect("You need to write a sentence, preferably a long one")
        .replace("\n", "");
    let mut input = input_str.as_bytes();
    let mmap;
    if let Some(filename) = std::env::args().nth(4) {
        if &input == b"-f" {
            mmap = unsafe {
                MmapOptions::new()
                    .map(&std::fs::File::open(&filename).unwrap())
                    .unwrap()
            };
            input = &mmap[..];
        }
    };
    let indices = (0..input.len()).step_by(500).collect::<Vec<_>>();
    let hashmaps: Vec<HashMap<String, f64>> = indices.into_par_iter()
        .map(|i| conf(&input[i..std::cmp::min(i+500, input.len())], average_length))
        .collect();
    let total_length = hashmaps.len() as f64;
    let combined: HashMap<String, f64> = hashmaps.into_iter().reduce(
            |mut acc, curr| {
                for key in curr.keys() {
                    let entr = acc.entry(key.clone()).or_insert(0.0);
                    *entr += curr.get(key).unwrap();
                }
                acc
            },
        ).unwrap();
    let mut sorted = combined.into_iter().collect::<Vec<_>>();
    sorted.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    sorted.iter().rev().take(top_n).for_each(|(k, v)| {
      println!("{}: {}", k, v / total_length)
    });
}

fn conf(input_bytes: &[u8], average_length: f64) -> HashMap<String, f64> {
    let compressed_length = compress_prepend_size(input_bytes).len();
    let mut compression_ratios = LENGTHS
        .par_iter()
        .map(|(lang, (f1, compr_f1_len))| {
            let f2 = input_bytes;
            //let ratio = (*compr_f1_len as f64 - averaged_compressed_length) / (averaged_compressed_length);
            let result = compression_ratio(f1, f2, *compr_f1_len, compressed_length).unwrap();
            (result, lang.as_str())
        })
        .collect::<Vec<(f64, &str)>>();
    compression_ratios.sort_by(|&(f1, _), &(f2, _)| f1.partial_cmp(&f2).unwrap());

    let (f1, _lang) = compression_ratios.get(0).expect("No languages found");
    let f1 = 1.0 - *f1;
    let mut adjusted_ratios = HashMap::from_iter(
        compression_ratios
            .iter()
            .map(|(confidence, lang)| ((1.0 - confidence) / f1, lang))
            .map(|(confidence, lang)| {
                let length_ratio =
                    LENGTHS.get(&lang.to_string()).unwrap().1 as f64 / average_length;
                let confidence = confidence * length_ratio;
                (lang.to_string(), confidence)
            }),
    );
    adjusted_ratios
}
