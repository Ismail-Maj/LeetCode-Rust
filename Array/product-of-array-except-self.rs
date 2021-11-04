// https://leetcode.com/problems/product-of-array-except-self/

struct Solution;

impl Solution {
    // O(n) time - O(1) extra space
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![1; nums.len()];
        let mut left = 1;
        for i in 0..nums.len() {
            res[i] *= left;
            left *= nums[i];
        }
        let mut right = 1;
        for i in (0..nums.len()).rev() {
            res[i] *= right;
            right *= nums[i];
        }
        res
    }
}