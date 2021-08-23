// https://leetcode.com/problems/longest-increasing-subsequence/

struct Solution

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut res: Vec<i32> = vec![];
        nums.iter().for_each(|num| {
            if let Err(index) = res.binary_search(num) { //Err means num isn't in the array
                if index == res.len() {
                    res.push(*num);
                }else{
                    res[index] = *num;
                }
            }
        });
        res.len() as i32
    }
}