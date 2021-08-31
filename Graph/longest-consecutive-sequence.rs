// https://leetcode.com/problems/longest-consecutive-sequence/

struct Solution;


use std::collections::HashSet;
use std::iter::FromIterator;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut res: i32 = 0;
        let set: HashSet<i32> = HashSet::from_iter(nums);
        for &e in &set {
            if !set.contains(&(e - 1)) {
                let mut i = 1;
                while set.contains(&(e + i)) {
                    i += 1;
                }
                res = res.max(i);
            }
        }
        res
    }
}