use std::collections::{BTreeSet, HashSet};
pub struct Solution;

impl Solution {
    // With HashMap<i32,()> there is incredibly small chance of hash collision so one would need HashMap<i32,i32> that
    // put a as both key and value and then compares values. Which is exactly what Hashset is under
    // the hood
    pub fn hash_simple(nums: &[i32]) -> bool {
        let mut seen_elements = HashSet::new();
        for &a in nums {
            if !seen_elements.insert(a) {
                return true;
            }
        }
        return false;
    }
    pub fn hash_iter(nums: &[i32]) -> bool {
        let mut seen = HashSet::new();
        nums.iter().any(|&a| !seen.insert(a))
    }
    // suboptimal to hashset
    pub fn bst_iter(nums: &[i32]) -> bool {
        let mut seen = BTreeSet::new();
        nums.iter().any(|&a| !seen.insert(a))
    }
    // Sorting way O(N log N) but with O(1) space
    // requires mutable vector to sort it in place
    pub fn sort_simple(mut nums: Vec<i32>) -> bool {
        nums.sort_unstable();
        nums.windows(2).any(|w| w.get(0) == w.get(1))
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let num = vec![1, 2, 3, 1];
        let num2 = vec![1, 2, 3, 4];
        assert_eq!(Solution::hash_simple(&num), true);
        assert_eq!(Solution::hash_simple(&num2), false);
    }
    #[test]
    fn test_iter() {
        let num = vec![1, 2, 3, 1];
        let num2 = vec![1, 2, 3, 4];
        assert_eq!(Solution::hash_iter(&num), true);
        assert_eq!(Solution::hash_iter(&num2), false);
    }

    fn test_suite(f: fn(&[i32]) -> bool) {
        let num = vec![1, 2, 3, 1];
        let num2 = vec![1, 2, 3, 4];
        assert_eq!(f(&num), true);
        assert_eq!(f(&num2), false);
    }

    #[test]
    fn test_hash_simple() {
        test_suite(Solution::hash_simple);
    }
    #[test]
    fn test_hash_iter() {
        test_suite(Solution::hash_iter);
    }
    #[test]
    fn test_bst() {
        test_suite(Solution::bst_iter);
    }

    // Macro way
    macro_rules! generate_tests {
        ($($name:ident),* => $version:literal) => {
           $(
            paste::paste! {
                #[test]
                fn [< test_ $name _ $version >](){
                    let num = vec![1, 2, 3, 1];
                    let num2 = vec![1, 2, 3, 4];
                    let f = Solution::$name;
                    assert_eq!(f(&num),true);
                    assert_eq!(f(&num2),false);
                }
            }
           )*
        };
    }

    generate_tests!(hash_simple, hash_iter => 2);
}
pub fn main() {}
