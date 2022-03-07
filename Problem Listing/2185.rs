// https://leetcode.com/problems/counting-words-with-a-given-prefix/
// 8 ms, 2.1 MB

impl Solution {
    pub fn prefix_count(words: Vec<String>, pref: String) -> i32 {
        let mut res = 0;
        'outer: for word in words {
            if word.len() < pref.len() {
                continue 'outer;
            }
            for c in 0..pref.len() {
                if word.chars().nth(c).unwrap() != pref.chars().nth(c).unwrap() {
                    continue 'outer;
                }
            }
            res += 1;
        }
        return res;
    }
}
