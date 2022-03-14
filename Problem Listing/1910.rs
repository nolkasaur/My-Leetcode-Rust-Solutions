// https://leetcode.com/problems/remove-all-occurrences-of-a-substring/
// 0 ms, 2.6 MB

impl Solution {
    pub fn remove_occurrences(mut s: String, part: String) -> String {
        let mut size = s.len();
        loop {
            s = s.replacen(&part, "", 1);
            if s.len() == size {
                break;
            } else {
                size = s.len();
            }
        }
        return s;
    }
}
