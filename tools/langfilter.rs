#!/usr/bin/env rust-script -o
//!
//! ```cargo
//! [dependencies]
//! lz4_flex = { version = "0.9.0", default-features = false }
//! rayon = "1.5.3"
//! memmap = "0.7.0"
//! term_macros = { git = "https://github.com/langwitch-tools/langwitch-preprocessing-toolkit" }
//! ```

use term_macros::*;
use std::collections::HashMap;
use lz4_flex::compress_prepend_size;
use memmap::MmapOptions;
use rayon::prelude::*;
use std::io::{Error, ErrorKind};

fn main() {

    tool! {
        args:
            - desired_lang: String;
            - reference_files: String;
            - min_confidence: f64;
            - sparsity: usize = 30;
        ;

        body: || {

            let lengths = get_lengths(&desired_lang, &reference_files, &sparsity);
            let average_length = lengths
                .values()
                .map(|(_, byte_length)| byte_length)
                .sum::<usize>() as f64
                / lengths.len() as f64;

            filter_in!(|sentence: &[u8]| {
                let top = get_likelihood_of_lang(&lengths, sentence, average_length, &desired_lang);
                if top > min_confidence {
                    return true;
                }
                false
            });
        }

    };
}

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

fn get_lengths(desired_lang: &str, language_dir: &str, sparsity: &usize) -> HashMap<String, (memmap::Mmap, usize)> {
    let files = get_files(language_dir)
        .unwrap()
        .into_iter()
        .enumerate()
        .filter(|(i, c)| i % sparsity == 0 || c.to_string_lossy().to_string() == desired_lang)
        .map(|(_i, c)| c)
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
}

pub fn get_likelihood_of_lang(lengths: &HashMap<String, (memmap::Mmap, usize)>, input_bytes: &[u8], average_length: f64, desired_lang: &str) -> f64 {

    let compressed_length = compress_prepend_size(input_bytes).len();
    let mut compression_ratios = lengths
        .par_iter()
        .map(|(lang, (f1, compr_f1_len))| {
            let f2 = input_bytes;
            //let ratio = (*compr_f1_len as f64 - averaged_compressed_length) / (averaged_compressed_length);
            let result = compression_ratio(f1, f2, *compr_f1_len, compressed_length).unwrap();
            (result, lang.as_str())
        })
        .collect::<Vec<(f64, &str)>>();
    let (f1, _lang) = compression_ratios.iter().max_by(|&(f1, _), &(f2, _)| f1.partial_cmp(&f2).unwrap()).unwrap();

    //let (f1, _lang) = compression_ratios.get(0).expect("No languages found");
    let f1 = 1.0 - *f1;
    let mut adjusted_ratios = HashMap::new();
    compression_ratios
        .iter()
        .map(|(confidence, lang)| ((1.0 - confidence) / f1, lang))
        .map(|(confidence, lang)| {
            let length_ratio = lengths.get(&lang.to_string()).unwrap().1 as f64 / average_length;
            let confidence = confidence * length_ratio;
            (*lang, confidence)
        })
        .for_each(|(lang, conf)| {adjusted_ratios.insert(lang, conf);});

    *adjusted_ratios.get(desired_lang).expect("the language you specified wasn't in the corpus. dummy")
}
