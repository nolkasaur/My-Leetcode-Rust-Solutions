// https://leetcode.com/problems/number-of-steps-to-reduce-a-number-to-zero/
// 0 ms, 2.2 MB

impl Solution {
    pub fn number_of_steps(num: i32) -> i32 {
        let mut r = num;
        let mut count = 0;
        while r != 0 {
            if r % 2 == 0 { r /= 2; }
            else { r -= 1; }
            count += 1;
        }
        return count;
    }
}
