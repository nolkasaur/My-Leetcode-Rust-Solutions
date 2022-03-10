// https://leetcode.com/problems/length-of-last-word/
// 3 ms, 2.2 MB

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let trimmed = s.trim_end();
        let mut res = 0;
        let mut pos: i32 = (trimmed.len()-1) as i32;
        while pos >= 0 && trimmed.chars().nth(pos as usize).unwrap() != ' ' {
            res += 1;
            pos -= 1;
        }
        return res;
    }
}
