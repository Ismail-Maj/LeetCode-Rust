// https://leetcode.com/problems/maximum-product-subarray/

struct Solution;

impl Solution {
    // O(n) time - O(1) space
    pub fn max_product(nums: Vec<i32>) -> i32 {
        nums.iter()
            .fold((1, 1, i32::MIN), |(min, max, best), &num| {
                let (mut new_min, mut new_max) = (min * num, max * num);
                if new_min > new_max {
                    std::mem::swap(&mut new_min, &mut new_max);
                }
                (i32::min(1, new_min), i32::max(1, new_max), i32::max(best, new_max))
            })
            .2
    }
}
