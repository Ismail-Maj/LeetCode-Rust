// https://leetcode.com/problems/two-sum/

struct Solution;

impl Solution {
    // O(n) time - O(n) space
    // O(n) time - O(1) space can be done using two pointers if the array is sorted (same idea as 3sum)
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut m: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();

        for (i1, &num) in nums.iter().enumerate() {
            let i1 = i1 as i32;

            match m.get(&(target - num)) {
                Some(&i2) => return vec![i2, i1],
                None => m.insert(num, i1),
            };
        }
        unreachable!()
    }
}
