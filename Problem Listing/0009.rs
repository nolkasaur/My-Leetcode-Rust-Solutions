// https://leetcode.com/problems/palindrome-number/
// 29 ms, 2.1 MB

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        return x.to_string() == x.to_string().chars().rev().collect::<String>();
    }
}
