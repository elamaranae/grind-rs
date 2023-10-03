/*
  Given two strings s and t, return true if t is an anagram of s, and false otherwise. An Anagram
  is a word or phrase formed by rearranging the letters of a different word or phrase, typically
  using all the original letters exactly once.
*/

use std::collections::HashMap;

pub fn run(s: String, t: String) -> bool {
    let mut frequency: HashMap<char, i32> = HashMap::new();
    frequency = s.chars().fold(frequency, |mut map, c| {
        *map.entry(c).or_insert(0) += 1;
        map
    });

    frequency = t.chars().fold(frequency, |mut map, c| {
        *map.entry(c).or_insert(0) -= 1;
        map
    });

    !frequency.values().any(|&v| v != 0)
}

#[test]
fn case_0() {
    assert!(run(String::from("anagram"), String::from("nagaram")));
}


#[test]
fn case_1() {
    assert!(!run(String::from("rat"), String::from("car")));
}

#[test]
fn case_3() {
    assert!(run(String::new(), String::new()));
}
