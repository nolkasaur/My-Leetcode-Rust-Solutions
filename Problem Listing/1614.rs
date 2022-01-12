// https://leetcode.com/problems/maximum-nesting-depth-of-the-parentheses/
// 0 ms, 1.9 MB

impl Solution {
    pub fn max_depth(s: String) -> i32 {
        let mut parBal = 0;
        let mut maxParBal = 0;
        for x in s.chars() {
            if x == '(' { parBal+=1; }
            else if x == ')' { parBal-=1; }
            if parBal>maxParBal { maxParBal = parBal; }
        }
        return maxParBal;
    }
}
