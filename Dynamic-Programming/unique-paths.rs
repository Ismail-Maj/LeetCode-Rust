// https://leetcode.com/problems/unique-paths/

struct Solution;

impl Solution {
    fn unique_paths(m: i32, n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;
        let mut dp = vec![vec![0; m + 1]; n + 1];
        for i in 1..=n {
            for j in 1..=m {
                if i == 1 && j == 1 {
                    dp[i][j] = 1;
                } else {
                    dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
                }
            }
        }
        dp[n][m]
    }
}