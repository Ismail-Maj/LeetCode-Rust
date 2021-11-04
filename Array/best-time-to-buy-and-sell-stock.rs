// https://leetcode.com/problems/best-time-to-buy-and-sell-stock/

struct Solution;

use std::cmp::Ordering;

impl Solution {
    // O(n) time - O(1) space
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() == 0 {
            return 0;
        }

        let mut max_profit: i32 = 0;
        let mut cheapest: i32 = prices[0];

        prices.iter().for_each(|price| match cheapest.cmp(price) {
            Ordering::Less => max_profit = max_profit.max(price - cheapest),
            Ordering::Greater => cheapest = *price,
            Ordering::Equal => (),
        });
        max_profit
    }
}
