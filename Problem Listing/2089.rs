// https://leetcode.com/problems/find-target-indices-after-sorting-array/
// 5 ms, 2.2 MB

impl Solution {
    pub fn target_indices(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut sorted = nums;
        sorted.sort();
        let mut res: Vec<i32> = vec![];
        for x in 0..sorted.len() {
            if sorted[x] == target {
                res.push(x as i32);
            }
        }
        return res;
    }
}
