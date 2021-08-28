// https://leetcode.com/problems/combination-sum-iv/

struct Solution

impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let mut dp: Vec<i32> = vec![0; target as usize +1];
        dp[0] = 1;
        for i in 0..=target {
            if dp[i as usize] > 0 {
                nums.iter().for_each(|&num| {
                    if i+num <= target {
                        dp[(i+num) as usize] += dp[i as usize];
                    }
                }); 
            }
        }
        dp[target as usize]
    }
}