// https://leetcode.com/problems/minimum-add-to-make-parentheses-valid/
// 0 ms, 2.1 MB

impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let mut res = 0;
        let mut balance = 0;
        for parentheses in s.chars() {
            if parentheses == '(' {
                if balance < 0 {
                    balance = 0;
                }
                balance += 1;
            } else {
                balance -= 1;
            }
            if balance < 0 {
                res += 1;
            }
        }
        if balance > 0 {
            return res + balance;
        } else {
            return res;
        }
    }
}
