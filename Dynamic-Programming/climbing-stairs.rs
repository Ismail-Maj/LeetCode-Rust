// https://leetcode.com/problems/climbing-stairs/

struct Solution;

impl Solution {
    // O(n) time - O(1) space - dynamic programming
    pub fn climb_stairs(n: i32) -> i32 {
        let mut previous = 0;
        let mut current = 1;
        (0..n).for_each(|_| {
            let next = current + previous;
            previous = current;
            current = next;
        });
        current
    }
}