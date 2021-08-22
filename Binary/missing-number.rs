https://leetcode.com/problems/missing-number/

struct Solution

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        (nums.len() * (nums.len()+1) / 2) as i32 - nums.iter().sum::<i32>()
    }
}