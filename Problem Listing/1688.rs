// https://leetcode.com/problems/count-of-matches-in-tournament/
// 0 ms, 2 MB

impl Solution {
    pub fn number_of_matches(n: i32) -> i32 {
        let mut res = 0;
        let mut c = n;
        while c/2 != 0 {
            if c%2 == 0 {
                res+=c/2;
                c/=2;
            } else {
                res+=(c/2);
                c=(c/2)+1;
            }
        }
        return res;
    }
}
