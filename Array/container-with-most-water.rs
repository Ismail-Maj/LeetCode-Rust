// https://leetcode.com/problems/container-with-most-water/

struct Solution;

impl Solution {
    // O(n) time - O(1) space - two pointers
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max_area = 0;

        let (mut l, mut r) = (0, height.len() - 1);
        while l < r {
            let min = i32::min(height[l], height[r]);
            max_area = i32::max(max_area, (min * (r - l) as i32));
            while l < r && height[l] <= min {l += 1;}
            while l < r && height[r] <= min {r -= 1;}
        }
        max_area
    }
}
