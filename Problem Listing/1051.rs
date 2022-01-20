// https://leetcode.com/problems/height-checker/
// 3 ms, 2.1 MB

impl Solution {
    pub fn height_checker(heights: Vec<i32>) -> i32 {
        let mut sorted = heights.clone();
        sorted.sort();
        let mut res = 0;
        for x in 0..heights.len() {
            if heights[x] != sorted[x] { res+=1; }
        }
        return res;
    }
}
