// https://leetcode.com/problems/a-number-after-a-double-reversal/submissions/
// 0 ms, 2 MB

impl Solution {
    pub fn is_same_after_reversals(num: i32) -> bool {
        return (num == 0 || num % 10 != 0)
    }
}
