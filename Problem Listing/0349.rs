// https://leetcode.com/problems/intersection-of-two-arrays/
// 0 ms, 2.2 MB

use std::cmp::min;

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut res:Vec<i32> = Vec::new();
        let mut dedup1 = nums1;
        let mut dedup2 = nums2;
        dedup1.sort();
        dedup2.sort();
        dedup1.dedup();
        dedup2.dedup();
        let mut smallest = &dedup1;
        let mut biggest = &dedup2;
        let len1 = dedup1.len();
        match min(dedup1.len(), dedup2.len()) {
            len1 => {smallest = &dedup1; biggest = &dedup2;},
            _ => {smallest = &dedup2; biggest = &dedup1;},
        }
        for elem in smallest {
            if biggest.contains(&elem) {
                res.push(*elem);
            }
        }
        return res;
    }
}
