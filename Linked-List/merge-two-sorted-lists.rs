// https://leetcode.com/problems/merge-two-sorted-lists/
struct Solution;


impl Solution {
    // O(n) time - O(1) space
    pub fn merge_two_lists(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        let mut curr = &mut dummy;
        loop {
            match (l1, l2) {
                (Some(mut n1), Some(mut n2)) => {
                    let (big, mut small) = if (n1.val > n2.val) { 
						(n1, n2)
					} else {
						(n2, n1)
					};
                    l1 = small.next.take();
                    l2 = Some(big);
                    curr.next = Some(small);
                },
                (Some(mut n), None) | (None, Some(mut n)) => {
					curr.next = Some(n); 
					break;
				},
                (None, None) => break
            }
            curr = curr.next.as_mut().unwrap();
        }
        dummy.next
    }


    // O(n) time - O(n) memory in the call stack
    pub fn merge_two_lists_recursive(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (None, None) => None,
            (Some(n), None) | (None, Some(n)) => Some(n),
            (Some(l1), Some(l2)) => {
                if l1.val < l2.val {
                    Some(Box::new(ListNode {
                        val: l1.val,
                        next: Solution::merge_two_lists(l1.next, Some(l2))
                    }))
                } else {
                    Some(Box::new(ListNode {
                        val: l2.val,
                        next: Solution::merge_two_lists(Some(l1), l2.next)
                    }))
                }
            }
        }
    }
}
