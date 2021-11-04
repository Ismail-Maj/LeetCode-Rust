// https://leetcode.com/problems/contains-duplicate/

struct Solution;

use std::collections::HashSet;

impl Solution {
    // O(n) time - O(n) space - hash table.
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut s: HashSet<i32> = HashSet::new();

        for num in nums {
            if s.contains(&num) {
                return true;
            } else {
                s.insert(num);
            }
        }
        false
    }
}