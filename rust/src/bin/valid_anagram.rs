use std::{collections::HashMap, iter::Zip, string};

pub struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut counts = HashMap::new();

        for (a, b) in s.chars().zip(t.chars()) {
            *counts.entry(a).or_insert(0) += 1;
            *counts.entry(b).or_insert(0) -= 1;
        }

        counts.values().all(|&val| val == 0)
    }
    pub fn is_anagram_optimized(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut counts = [0; 26];

        s.bytes().zip(t.bytes()).for_each(|(a, b)| {
            counts[(a - b'a') as usize] += 1;
            counts[(b - b'a') as usize] -= 1;
        });

        counts.iter().all(|&val| val == 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    macro_rules! generate_tests {
        ($($name:ident),* => $version:literal) => {
           $(
            paste::paste! {
                #[test]
                fn [< test_ $name _ $version >](){
                    let word= String::from("abccf");
                    let word2 = String::from("acbcf");
                    let word3 = String::from("abdcf");
                    let f = Solution::$name;
                    assert_eq!(f(word.clone(),word2),true);
                    assert_eq!(f(word,word3),false);
                }
            }
           )*
        };
    }

    generate_tests!(is_anagram, is_anagram_optimized => 1);
}
pub fn main() {}
