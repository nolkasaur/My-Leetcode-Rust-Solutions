// https://leetcode.com/problems/remove-outermost-parentheses/
// 2 ms, 2.1 MB

impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let mut bal = 0;
        let mut res = String::from("");
        for x in s.chars() {
            if x == '(' && bal == 0 {
                bal += 1;
            } else if x == '(' && bal > 0 {
                res.push('(');
                bal += 1;
            } else if x == ')' && bal > 1 {
                res.push(')');
                bal -= 1;
            } else {
                bal -= 1;
            }
        }
        return res;
    }
}
