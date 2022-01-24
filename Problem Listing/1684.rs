// https://leetcode.com/problems/count-the-number-of-consistent-strings/
// 16 ms, 2.7 MB

impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        let mut res = 0;
        'outer: for word in words {
            for char in word.chars() {
                if !allowed.contains(char) { continue 'outer }
            }
            res+=1;
        }
        return res;
    }
}
