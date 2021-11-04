// https://leetcode.com/problems/maximum-subarray/

struct Solution;

impl Solution {
    // O(n) time - O(1) space
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        nums.iter()
            .fold((0, i32::MIN), |(current, best), &num| {
                (i32::max(0, current + num), best.max(current + num))
            }).1
    }
}
