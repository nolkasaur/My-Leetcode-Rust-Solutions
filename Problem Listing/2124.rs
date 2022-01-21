// https://leetcode.com/problems/check-if-all-as-appears-before-all-bs/
// 2 ms, 2.1 MB

impl Solution {
    pub fn check_string(s: String) -> bool {
        let mut foundB = false;
        for x in s.chars() {
            if x == 'b' { foundB = true }
            else if foundB { return false }
        }
        return true
    }
}
