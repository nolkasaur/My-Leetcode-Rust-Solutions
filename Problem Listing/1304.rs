// https://leetcode.com/problems/find-n-unique-integers-sum-up-to-zero/
// 2 ms, 2 MB

impl Solution {
    pub fn sum_zero(n: i32) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::with_capacity(n as usize);
        let mut c = n;
        for x in 0..n/2 {
            res.push(c);
            res.push(c*(-1));
            c-=1;
        }
        if n%2!=0 {
            res.push(0);
        }
        return res;
    }
}
