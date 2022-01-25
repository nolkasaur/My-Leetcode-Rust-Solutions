// https://leetcode.com/problems/distribute-candies/submissions/
// 54 ms, 2.2 MB

use std::cmp::min;

impl Solution {
    pub fn distribute_candies(candy_type: Vec<i32>) -> i32 {
        let allowed = candy_type.len()/2;
        let mut dedup = candy_type;
        dedup.sort();
        dedup.dedup();
        return min(allowed as i32, dedup.len() as i32)
    }
}
