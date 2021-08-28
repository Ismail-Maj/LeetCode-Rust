// https://leetcode.com/problems/jump-game/

struct Solution;

use std::cmp::max;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut max_reach: usize = 0;
        for i in 0..nums.len() {
            if max_reach < i { return false; }
            max_reach = max(max_reach, i+nums[i] as usize);
        }
        true
    }
}