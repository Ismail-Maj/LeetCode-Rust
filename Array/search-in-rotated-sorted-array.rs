impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();

        let n: usize = nums.len();
        
        if n < 3 {
            return vec![];
        }
        let mut res: Vec<Vec<i32>> = vec![];

        for i in 0..n-2 {
            
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            
            let (mut j, mut k) = (i+1, n-1);
            while j < k {
                let total = nums[i] + nums[j] + nums[k];

                if total <= 0 {
                    j += 1;
                }
                if total >= 0 {
                    k -= 1;
                }
                if total == 0 {
                    res.push(vec![nums[i], nums[j-1], nums[k+1]]);
                    while j < k && nums[j] == nums[j - 1] {j += 1;}
                    while j < k && nums[k] == nums[k + 1] {k -= 1;}
                }

            }
        }
        res
    }
}