// https://leetcode.com/problems/linked-list-cycle/

struct Solution;

impl Solution {
    pub fn hasCycle(ll: std::collections::LinkedList<i32>) -> bool {
        let mut slow = ll.iter();
        let mut fast = ll.iter();

        loop {
            let slow_val = if let Some(&v) = slow.next() {v} else { break };
            fast.next();
            let fast_val = if let Some(&v) = fast.next() {v} else { break };
            if slow_val == fast_val {
                return true;
            }
        }
        false
    }
}

