// https://leetcode.com/problems/merge-strings-alternately/
// 2 ms, 2 MB

use std::cmp;

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut res = String::from("");
        let chars1:Vec<char> = word1.chars().collect();
        let chars2:Vec<char> = word2.chars().collect();
        let minLen = cmp::min(word1.len(), word2.len());
        let maxLen = cmp::max(word1.len(), word2.len());
        for x in 0..minLen {
            res.push(chars1[x]);
            res.push(chars2[x]);
        }
        if minLen == word1.len() {
            for y in minLen..maxLen {
                res.push(chars2[y]);
            }
        } else {
            for y in minLen..maxLen {
                res.push(chars1[y]);
            }
        }
        return res;
    }
}
