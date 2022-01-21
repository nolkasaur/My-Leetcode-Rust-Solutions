// https://leetcode.com/problems/check-if-numbers-are-ascending-in-a-sentence/
// 2 ms, 2.1 MB

impl Solution {
    pub fn are_numbers_ascending(s: String) -> bool {
        let mut prevNum = 0;
        for x in s.split(' ') {
            let num = match x.parse::<i32>() {
                Ok(val) => val,
                Err(err) => -1 as i32,
            };
            if num != -1 as i32 {
                if num <= prevNum { return false }
                else { prevNum = num }
            }
        }
        return true
    }
}
