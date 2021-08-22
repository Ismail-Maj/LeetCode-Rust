// https://leetcode.com/problems/3sum/

struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();

        let n: usize = nums.len();
        if n == 0 {
            return vec![];
        }
        let mut s: HashSet<(i32, i32, i32)> = HashSet::new();
        for i in 0..n-1{
            let (mut j, mut k) = (i+1, n-1);
            while j < k {
                let candidate = nums[i] + nums[j] + nums[k];
                if candidate == 0 {
                    s.insert((nums[i], nums[j], nums[k]));
                    j+=1;
                    k-=1;
                } else if candidate < 0 {
                    j+=1;
                }else{
                    k-=1;
                }
            }
        }
        s.iter().map(|x| vec![x.0, x.1, x.2]).collect()
    }
}