// https://leetcode.com/problems/counting-bits/
// 6 ms, 2.5 MB

impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut res = Vec::with_capacity((n+1) as usize);
        for x in 0..n+1 {
            if x == 0 {
                res.push(0);
            } else if x == 1 {
                res.push(1);
            } else {
                res.push(res[(x/2) as usize] + x%2);
            }
        }
        return res;
    }
}
