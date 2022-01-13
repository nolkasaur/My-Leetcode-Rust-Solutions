// https://leetcode.com/problems/find-the-highest-altitude/
// 2 ms, 2 MB

impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let mut net = 0;
        let mut res = 0;
        for x in 0..gain.len() {
            net+=gain[x];
            if net > res { res = net; }
        }
        return res;
    }
}
