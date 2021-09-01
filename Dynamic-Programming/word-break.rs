// https://leetcode.com/problems/word-break/

struct Solution;

use std::collections::HashSet;
use std::iter::FromIterator;

impl Solution {
    fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let n = s.len();
        let mut dp = vec![false; n + 1];
        let set: HashSet<String> = HashSet::from_iter(word_dict);
        dp[0] = true;
        for i in 1..=n {
            for j in 0..i {
                if dp[j] && set.contains(&s[j..i]) {
                    dp[i] = true;
                    break;
                }
            }
        }
        dp[n]
    }
}