// https://leetcode.com/problems/meeting-rooms/

use std::thread::current;

struct Solution;

impl Solution {
    pub fn can_attend_meetings(mut intervals: Vec<Vec<i32>>) -> bool {
        intervals.sort_by(|x, y| x[0].cmp(&y[0]));
        
        let mut furthest = i32::MIN;
        for interval in intervals {
            if furthest > interval[0] {
                return false;
            }
            furthest = furthest.max(interval[1]);
        }
        true
    }
}