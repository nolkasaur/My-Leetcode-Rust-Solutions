// https://leetcode.com/problems/calculate-money-in-leetcode-bank/
// 0 ms, 1.9 MB

impl Solution {
    pub fn total_money(n: i32) -> i32 {
        let mut mondayVal = 1;
        let mut addVal = 1;
        let mut res = 1;
        for x in 1..n {
            if x % 7 == 0 {
                mondayVal += 1;
                addVal = mondayVal;
            } else {
                addVal += 1;
            }
            res += addVal;
        }
        return res;
    }
}
