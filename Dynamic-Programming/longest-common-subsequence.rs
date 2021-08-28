// https://leetcode.com/problems/longest-common-subsequence/

struct Solution;

use std::cmp::{max};

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let mut dp : Vec<Vec<i32>> = vec![vec![0;text2.len()]; text1.len()];
        text1.chars().enumerate().for_each(|(idx1, c1)| {
            text2.chars().enumerate().for_each(|(idx2, c2)| {
                if c1 == c2 {
                    dp[idx1][idx2] = if (idx1 > 0) & (idx2 > 0) {dp[idx1-1][idx2-1]} else {0} + 1;
                }else{
                    dp[idx1][idx2] = max(if idx1 > 0 {dp[idx1-1][idx2]} else {0}, if idx2 > 0 {dp[idx1][idx2-1]} else {0});
                }
            });
        });
        dp[text1.len()-1][text2.len()-1]
    }
}