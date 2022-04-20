// https://leetcode.com/problems/reverse-string/
// 27 ms, 5.4 MB

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let len = s.len();
        for x in 0..len/2 {
            let oppositeChar = s[len-1-x];
            s[len-1-x] = s[x];
            s[x] = oppositeChar;
        }
    }
}
