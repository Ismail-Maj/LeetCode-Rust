// https://leetcode.com/problems/number-of-1-bits/

struct Solution;

impl Solution {
    // O(1) time - O(1) space
    pub fn hammingWeight (n: u32) -> i32 {
        let mut res :i32 = 0;
        let mut n = n as i32;
        while n != 0 {
            n = n & (n-1); // clear the least significant bit set to 0
            res+=1;
        }
        return res;
    }
}