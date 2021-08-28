// https://leetcode.com/problems/number-of-1-bits/

struct Solution;

impl Solution {
    pub fn hammingWeight (n: u32) -> i32 {
        let mut res :i32 = 0;
        let mut n = n as i32;
        while n != 0 {
            n = n & (n-1);
            res+=1;
        }
        return res;
    }
}