// https://leetcode.com/problems/product-of-array-except-self/

struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut acc: i32 = 1;
        let mut res: Vec<i32> = vec![0; nums.len()];
        
        nums.iter().zip(res.iter_mut()).for_each(|(num, el)| {
            *el  = acc;
            acc *= num;
        });
        
        acc = 1;     
        nums.iter().zip(res.iter_mut()).rev().for_each(|(num, el)| {
            *el *= acc;
            acc *= num;
        });
        
        res
    }
}