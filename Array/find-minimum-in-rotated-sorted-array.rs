// https://leetcode.com/problems/find-minimum-in-rotated-sorted-array/

struct Solution;

impl Solution {
    // O(log(n)) time - O(1) space - binary search
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let (mut l, mut r) = (0, nums.len() - 1);
        while l < r {
            let m = (l + r) / 2;
            if nums[m] > nums[r] {
                l = m + 1;
            } else {
                r = m;
            }
        }
        nums[l]
    }
}
