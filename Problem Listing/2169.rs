// https://leetcode.com/problems/count-operations-to-obtain-zero/
// 3 ms, 2 MB

impl Solution {
    pub fn count_operations(mut num1: i32, mut num2: i32) -> i32 {
        let mut res = 0;
        while num1 > 0 && num2 > 0 {
            if num1 >= num2 {
                num1 -= num2;
            } else {
                num2 -= num1;
            }
            res += 1;
        }
        return res;
    }
}
