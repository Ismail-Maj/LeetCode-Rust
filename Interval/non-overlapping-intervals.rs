// https://leetcode.com/problems/non-overlapping-intervals/

struct Solution;

impl Solution {
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_by(|x, y| x[1].cmp(&y[1]));

        intervals.iter().enumerate().fold((0, i32::MIN), |(total, prev_end), (i, curr)| {
            if i == 0 || prev_end <= curr[0] {
                (total,  curr[1])
            } else {
                (total+1, prev_end)
            }
        }).0
    }
}