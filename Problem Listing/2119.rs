// https://leetcode.com/problems/a-number-after-a-double-reversal/submissions/
// 2 ms, 2 MB

impl Solution {
    pub fn is_same_after_reversals(num: i32) -> bool {
        if num == 0 { return true }
        else { return num % 10 != 0 }
    }
}
