use std::cmp::{min, max};

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut res: i32 = 0;
        let mut cheapest: i32 = std::i32::MAX;
        prices.iter().for_each(|&price| {
            cheapest = min(cheapest, price);
            res = max(res, price-cheapest);
        });
        res
    }
}