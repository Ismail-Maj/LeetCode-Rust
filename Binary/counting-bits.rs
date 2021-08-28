// https://leetcode.com/problems/counting-bits/

struct Solution;

impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut res : Vec<i32> = vec![0];
        
        for i in 1..=n {
            res.push(res[(i>>1) as usize] + i % 2);
        }
        res
    }
}