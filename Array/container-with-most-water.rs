// https://leetcode.com/problems/container-with-most-water/

struct Solution;

use std::cmp::{min, max};

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut res = 0;
        
        let (mut l, mut r) = (0, height.len()-1);
        while l < r {
            let mn = min(height[l], height[r]);
            res = max(res, mn * (r-l) as i32);
            while l < r && height[l] <= mn { l+=1; }
            while l < r && height[r] <= mn { r-=1; }
        }
        res
    }
}