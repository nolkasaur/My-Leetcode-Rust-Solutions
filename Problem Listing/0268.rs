// https://leetcode.com/problems/missing-number/
// 4 ms, 2.2 MB

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut sorted = nums;
        sorted.sort();
        for x in 0..sorted.len() {
            if sorted[x] != x as i32 { return x as i32}
        }
        return sorted.len() as i32;
    }
}
