// https://leetcode.com/problems/number-of-1-bits/
// 0 ms, 2.2 MB

impl Solution {
    pub fn hammingWeight (n: u32) -> i32 {
        let mut res = 0;
        for bit in format!("{:b}", n).chars() {
            if bit == '1' { res += 1 }
        }
        return res;
    }
}
