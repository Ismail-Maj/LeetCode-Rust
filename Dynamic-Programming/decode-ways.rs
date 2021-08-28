// https://leetcode.com/problems/decode-ways/

struct Solution;

impl Solution {
    fn num_decodings(s: String) -> i32 {
        let chars: Vec<u8> = s.bytes().collect();
        let n = s.len();
        
        let mut dp: Vec<i32> = vec![0; n + 1];
        dp[0] = 1;

        let mut prev_c: u8 = 0;
        for i in 0..n {
            let c = chars[i];
            if c != b'0' {
                dp[i+1] += dp[i];
            }
            if prev_c == b'1' || (prev_c == b'2' && c < b'7') {
                dp[i+1] += dp[i-1];
            }
            prev_c = c;
        }
        dp[n]
    }
}