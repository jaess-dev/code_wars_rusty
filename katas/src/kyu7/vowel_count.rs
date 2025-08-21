use std::collections::HashSet;

use crate::kyu7::vowel_count;

/// https://www.codewars.com/kata/54ff3102c1bad923760001f3/train/rust
///
/// Return the number (count) of vowels in the given string.
/// We will consider a, e, i, o, u as vowels for this Kata (but not y).
/// The input string will only consist of lower case letters and/or spaces.
fn get_count(string: &str) -> usize {
    // Do your magic here
    let mut vowel_count = 0;

    // string -> char
    // count = 0
    // count += 1 if char == vowl
    let vowels = HashSet::from(['a', 'e', 'i', 'o', 'u']);

    // functional approach
    vowel_count = 0;
    vowel_count = string
        .chars()
        .fold(0, |acc, c| 
            if vowels.contains(&c) { acc + 1 } else { acc });

    // procedural 
    vowel_count = 0;
    for c in string.chars(){
        if vowels.contains(&c) {
            vowel_count += 1;
        }
    }

    vowel_count
}

#[test]
fn my_tests() {
    assert_eq!(get_count("abracadabra"), 5);
}
