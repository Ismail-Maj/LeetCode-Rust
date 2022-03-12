// https://leetcode.com/problems/meeting-rooms-ii/

struct Solution;

impl Solution {
    pub fn min_meeting_rooms(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_by(|x, y| x[0].cmp(&y[0]));

        let mut heap = std::collections::BinaryHeap::new();
        let mut res = 0;
        for interval in intervals {
            while let Some(&element) = heap.peek() {
                if element <= -interval[0] {
                    heap.pop();
                } else {
                    break;
                }
            }
            heap.push(-interval[1]);
            res = res.max(heap.len() as i32);
        }
        res
    }
}