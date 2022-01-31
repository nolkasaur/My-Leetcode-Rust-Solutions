// https://leetcode.com/problems/count-odd-numbers-in-an-interval-range/
// 0 ms, 2.2 MB

impl Solution {
    pub fn count_odds(low: i32, high: i32) -> i32 {
        let mut a = (high-low+1);
        let mut b = high;
        let mut c = low;
        if high % 2 != 0 {
            b += 1;
        }
        if low % 2 != 0 {
            c -= 1;
        }
        
        return (b-c) / 2
    }
}
