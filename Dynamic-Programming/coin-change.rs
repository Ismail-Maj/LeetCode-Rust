// https://leetcode.com/problems/coin-change/

struct Solution;

use std::cmp::Ordering;

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let amount = amount as usize;
        let mut dp = vec![None; amount + 1];
        dp[0] = Some(0);
        for i in 1..=amount {
            dp[i] = coins.iter().filter_map(|&j| {
                    let j = j as usize;
                    match j.cmp(&i) {
                        Ordering::Greater => None,
                        _ => dp[i - j],
                    }
                }).min().map(|n| n + 1);
        }
        dp[amount].unwrap_or(-1)
    }
}