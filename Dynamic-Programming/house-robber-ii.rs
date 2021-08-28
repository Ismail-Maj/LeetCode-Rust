// https://leetcode.com/problems/house-robber-ii/

struct Solution;

use std::cmp::max;

impl Solution {

    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 0 {
            return 0;
        }
        if n == 1 {
            return nums[0];
        }
        Self::rob_general(&nums[0..n - 1]).max(Self::rob_general(&nums[1..n]))
    }

    pub fn rob_general(nums: &[i32]) -> i32 {
        let (mut ignore, mut pick) = (0 , 0);
        nums.iter().for_each(|&house| {
            let pick_current = ignore + house;
            ignore = max(ignore, pick);
            pick = pick_current;
        });
        
        std::cmp::max(ignore, pick)
    }
}