// https://leetcode.com/problems/check-if-two-string-arrays-are-equivalent/
// 2 ms, 2.3 MB

impl Solution {
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        let mut conc1 = String::from("");
        for x in 0..word1.len() {
            conc1 += &word1[x];
        }
        let mut conc2 = String::from("");
        for y in 0..word2.len() {
            conc2 += &word2[y];
        }
        return conc1 == conc2;
    }
}
