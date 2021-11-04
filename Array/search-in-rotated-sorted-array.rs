// https://leetcode.com/problems/search-in-rotated-sorted-array/

struct Solution;


impl Solution {
    // O(log(n)) time - O(1) space - binary search
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut l, mut r) : (i32, i32) = (0, nums.len() as i32 -1);
        while l <= r {
            let m: i32 = (r+l)/2;
            if nums[m as usize] == target { return m; }
            if nums[l as usize] <= nums[m as usize] {
                if nums[m as usize] >= target && nums[l as usize] <= target  {
                    r = m-1;
                }else{
                    l = m+1;
                }
            }else{
                if nums[r as usize] >= target && nums[m as usize] <= target  {
                    l = m+1;
                }else{
                    r = m-1;
                }   
            }
        }
        -1
    }
}