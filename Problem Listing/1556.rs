// https://leetcode.com/problems/thousand-separator/
// 3 ms, 2.1 MB

impl Solution {
    pub fn thousand_separator(n: i32) -> String {
        if n < 1000 { return n.to_string() }
        else {
            let mut res = String::from("");
            let c: Vec<char> = n.to_string().chars().collect();
            res.push(c[c.len()-1]);
            for x in 1..c.len() {
                if x % 3 == 0 {
                    res.push('.');
                }
                res.push(c[c.len()-1-x]);
            }
            return res.chars().rev().collect();
        }
    }
}
