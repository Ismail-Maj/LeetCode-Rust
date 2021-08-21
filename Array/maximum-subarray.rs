// https://leetcode.com/problems/maximum-subarray/

struct Solution;

use std::cmp::max;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        nums.iter().fold((0, i32::MIN), |(cur, mx), &num| {
            (max(0, cur + num) ,max(mx, cur + num))
        }).1
    }
}