// https://leetcode.com/problems/three-consecutive-odds/
// 1 ms, 2 MB

impl Solution {
    pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
        let mut count = 0;
        for num in arr {
            if num % 2 != 0 {
                count += 1;
                if count == 3 {
                    return true
                }
            } else {
                count = 0;
            }
        }
        return false
    }
}
