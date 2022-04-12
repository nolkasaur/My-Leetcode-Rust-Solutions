// https://leetcode.com/problems/counting-bits/
// 15 ms, 2.5 MB

impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::with_capacity((n+1) as usize);
        for x in 0..n+1 {
            let s = format!("{:b}", x);
            res.push(s.matches('1').count() as i32);
        }
        return res;
    }
}
