// https://leetcode.com/problems/house-robber/

struct Solution;

use std::cmp::max;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let (mut ignore, mut pick) = (0 , 0);
        nums.iter().for_each(|&house| {
            let pick_current = ignore + house;
            ignore = max(ignore, pick);
            pick = pick_current;
        });
        
        std::cmp::max(ignore, pick)
    }
}

