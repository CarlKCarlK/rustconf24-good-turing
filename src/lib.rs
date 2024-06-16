use std::{
    collections::{HashMap, HashSet},
    io::{BufRead, BufReader, Read},
};

use lazy_regex::regex;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn good_turning_js(data: &[u8]) -> Vec<u64> {
    let reader = BufReader::new(data);
    let (prediction, actual) = good_turning(reader);
    vec![prediction as u64, actual as u64]
}

pub fn good_turning<R: Read>(reader: BufReader<R>) -> (usize, usize) {
    let re = regex!(r"\b\w+\b");
    let mut word_to_count_even = HashMap::new();
    let mut word_set_odd = HashSet::new();

    for (index, line) in reader.lines().enumerate() {
        let line = line.expect("could not read line");
        // split into words with regex
        for word in re.find_iter(&line) {
            let word = word.as_str();
            if index % 2 == 0 {
                // count # of times a word occurs on the even lines
                *word_to_count_even.entry(word.to_string()).or_insert(0) += 1;
            } else {
                // odd lines
                word_set_odd.insert(word.to_string());
            }
        }
    }

    let singleton_count_even = word_to_count_even
        .iter()
        .filter(|(_, &count)| count == 1)
        .count();
    let only_odd_count = word_set_odd
        .into_iter()
        .filter(|word| !word_to_count_even.contains_key(word))
        .count();

    (singleton_count_even, only_odd_count)
}

// fn main() {
//     let file_name = env::args().nth(1).expect("no file name given");
//     let (singleton_count_even, only_odd_count) = good_turning(&file_name);

//     println!(
//         "Prediction (Words that appear exactly once on even lines): {}",
//         singleton_count_even
//     );
//     println!(
//         "Actual distinct words that appear only on odd lines: {}",
//         only_odd_count
//     );
// }

// test `process_file` on `./pg100.txt`. The answer is 10223 and 7967.
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use wasm_bindgen_test::wasm_bindgen_test;
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[test]
    fn test_process_file() {
        let file_name = "pg100.txt";
        let reader = BufReader::new(File::open(file_name).expect("could not open file"));
        let (prediction, actual) = good_turning(reader);
        assert_eq!(prediction, 10223);
        assert_eq!(actual, 7967);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_good_turning_js() {
        let data = include_bytes!("../pg100.txt");
        let result = good_turning_js(data);
        assert_eq!(result, vec![10223, 7967]);
    }
}
