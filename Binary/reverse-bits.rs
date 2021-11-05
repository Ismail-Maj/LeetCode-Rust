// https://leetcode.com/problems/reverse-bits/

struct Solution;

impl Solution {
    // O(1) time - O(1) space
    pub fn reverse_bits(mut x: u32) -> u32 {
        let mut res = 0;
        for _ in 0..32 {
            res = (res << 1) | (x&1);
            x = x >> 1;
        }
        res
    }

    // O(1) time - O(1) space
    pub fn reverse_bits(mut x: u32) -> u32 {
        x = (x >> 16) | (x << 16);
        x = ((x & 0xff00ff00) >> 8) | ((x & 0x00ff00ff) << 8);
        x = ((x & 0xf0f0f0f0) >> 4) | ((x & 0x0f0f0f0f) << 4);
        x = ((x & 0xcccccccc) >> 2) | ((x & 0x33333333) << 2);
        x = ((x & 0xaaaaaaaa) >> 1) | ((x & 0x55555555) << 1);
        x
    }
}