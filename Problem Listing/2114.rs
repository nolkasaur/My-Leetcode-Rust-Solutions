// https://leetcode.com/problems/maximum-number-of-words-found-in-sentences/
// 0 ms, 2.1 MB

impl Solution {
    pub fn most_words_found(sentences: Vec<String>) -> i32 {
        let mut res = 0;
        let it = sentences.iter();
        for x in it {
            let count = x.matches(' ').count()+1;
            if count > res {
                res = count;
            }
        }
        return res as i32;
    }
}
