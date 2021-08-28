// https://leetcode.com/problems/climbing-stairs/

struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut dp: Vec<i32> = vec![0; n as usize + 2];
        dp[0] = 0;
        dp[1] = 1;
        for i in 2..=(n+1) {
            dp[i as usize] = dp[(i-1) as usize] + dp[(i-2) as usize];
        }
        dp[n as usize + 1]
    }
}