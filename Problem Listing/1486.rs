// https://leetcode.com/problems/xor-operation-in-an-array/
// 0 ms, 2.2 MB

impl Solution {
    pub fn xor_operation(n: i32, start: i32) -> i32 {
        let mut res = 0;
        for x in 0..n {
            res ^= start + (2*x);
        }
        return res;
    }
}
