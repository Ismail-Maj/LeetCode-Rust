use std::cmp::{max, Ordering};

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() == 0 { return 0 }

        let mut res: i32 = 0;
        let mut cheapest: i32 = prices[0];
        
        prices.iter().for_each(|price| {
            match cheapest.cmp(price){
                Ordering::Less => res = max(res, price-cheapest),
                Ordering::Greater => cheapest = *price,
                Ordering::Equal => (),
            }
        });
        res
    }
}