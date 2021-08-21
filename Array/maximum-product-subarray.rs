// https://leetcode.com/problems/maximum-product-subarray/

struct Solution;

use std::cmp::{min, max};
use std::mem;

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        nums.iter().fold((1, 1, i32::MIN), |(mn, mx, best), &num| {
            let (mut new_mn, mut new_mx) = (mn*num, mx*num);
            if new_mn > new_mx { mem::swap(&mut new_mn, &mut new_mx); }

            (
                min(1, new_mn),
                max(1, new_mx),
                max(best, new_mx),
            )
        }).2
    }
}