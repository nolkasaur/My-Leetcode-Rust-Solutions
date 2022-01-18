// https://leetcode.com/problems/reverse-prefix-of-word/
// 0 ms, 2 MB

impl Solution {
    pub fn reverse_prefix(word: String, ch: char) -> String {
        if !word.contains(ch) { return word; }
        let mut res = String::from("");
        for x in 0..word.len() {
            if word.chars().nth(x).unwrap() == ch {
                for y in 0..x {
                    res.push(word.chars().nth(x-y).unwrap());
                }
                res.push(word.chars().nth(0).unwrap());
                for z in x+1..word.len() {
                    res.push(word.chars().nth(z).unwrap());
                }
                break;
            }
        }
        return res;
    }
}
