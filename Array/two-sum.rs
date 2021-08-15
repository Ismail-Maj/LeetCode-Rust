use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut m : HashMap<i32, i32> = HashMap::new();
        
        for (i1, &num) in nums.iter().enumerate() {
            let i1 = i1 as i32;
            
            match m.get(&(target-num)) {
                Some(&i2) => return vec![i2, i1],
                None => m.insert(num, i1),
            };
            
        }
        unreachable!()
    }
}