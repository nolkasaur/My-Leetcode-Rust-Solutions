// https://leetcode.com/problems/find-first-palindromic-string-in-the-array/
// 10 ms, 2.2 MB

impl Solution {
    pub fn first_palindrome(words: Vec<String>) -> String {
        for word in words {
            if word.chars().rev().collect::<String>() == word { return word }
        }
        return String::from("");
    }
}
