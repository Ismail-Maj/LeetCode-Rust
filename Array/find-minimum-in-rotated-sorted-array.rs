// https://leetcode.com/problems/find-minimum-in-rotated-sorted-array/

struct Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut h = nums.len() - 1;
        while l < h {
            if nums[l] < nums[h] {
                return nums[l];
            }
            let m = l + (h - l) / 2;
            if nums[l] <= nums[m] {
                l = m + 1;
            } else {
                h = m;
            }
        }
        nums[l]
    }
}
