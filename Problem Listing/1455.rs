// https://leetcode.com/problems/check-if-a-word-occurs-as-a-prefix-of-any-word-in-a-sentence/
// 0 ms, 2.2 MB

impl Solution {
    pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
        let mut split: Vec<&str> = sentence.split(' ').collect();
        'outer: for x in 0..split.len() {
            for c in 0..search_word.len() {
                if split[x].len() >= search_word.len() && split[x].chars().nth(c).unwrap() == search_word.chars().nth(c).unwrap() {
                    continue;
                } else {
                    continue 'outer;
                }
            }
            return (x + 1) as i32
        }
        return -1;
    }
}
