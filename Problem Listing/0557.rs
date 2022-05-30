// https://leetcode.com/problems/reverse-words-in-a-string-iii/
// 9 ms, 2.5 MB

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut splt: Vec<&str> = s.split(" ").collect();
        let mut res: Vec<String> = Vec::with_capacity(splt.len());
        for x in 0..splt.len() {
            let mut word = splt[x];
            let revWord: String = word.chars().rev().collect();
            res.push(revWord.to_owned());
        }
        return res.join(" ");
    }
}
