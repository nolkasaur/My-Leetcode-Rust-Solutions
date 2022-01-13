// https://leetcode.com/problems/replace-all-digits-with-characters/
// 0 ms, 2 MB

impl Solution {
    pub fn replace_digits(s: String) -> String {
        let mut res = String::new();
        for x in 0..s.len() {
            if x%2==0 {
                res.push(s.chars().nth(x).unwrap());
            } else {
                let digit = s.chars().nth(x).unwrap().to_digit(10).unwrap() as u8;
                let prev = s.chars().nth(x-1).unwrap() as u8;
                let shifted = (digit+prev) as char;
                res.push(shifted);
            }
        }
        return res;
    }
}
