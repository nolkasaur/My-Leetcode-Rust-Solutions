// https://leetcode.com/problems/number-of-strings-that-appear-as-substrings-in-word/
// 0 ms, 2.3 MB

impl Solution {
    pub fn num_of_strings(patterns: Vec<String>, word: String) -> i32 {
        let mut res = 0;
        for x in 0..patterns.len() {
            if word.contains(&patterns[x]) { res+=1; }
        }
        return res;
    }
}
