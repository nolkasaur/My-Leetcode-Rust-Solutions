// https://leetcode.com/problems/split-a-string-in-balanced-strings/
// 0 ms, 2 MB

impl Solution {
    pub fn balanced_string_split(s: String) -> i32 {
        let mut res = 0;
        let mut lBal = 0;
        for x in s.chars() {
            if x == 'L' {
                lBal+=1;
            } else {
                lBal-=1;
            }
            if lBal == 0 { res+=1; }
        }
        return res;
    }
}
