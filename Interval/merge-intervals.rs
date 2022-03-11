// https://leetcode.com/problems/merge-intervals/

struct Solution;

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if intervals.len() == 0 {
            return intervals;
        }
        
        intervals.sort_by(|x, y| x[0].cmp(&y[0]));
        let mut res: Vec<Vec<i32>> = vec![vec![intervals[0][0], intervals[0][1]]];
        
        for curr in intervals.into_iter().skip(1) {
            if let Some(prev) = res.last_mut() {
                if prev[1] >= curr[0] {
                    *prev = vec![prev[0], std::cmp::max(curr[1], prev[1])];
                } else {
                    res.push(curr);
                }
            }
        }
        
        res
    }
}