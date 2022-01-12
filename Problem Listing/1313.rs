// https://leetcode.com/problems/decompress-run-length-encoded-list/
// 15 ms, 2 MB

impl Solution {
    pub fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        let mut res: Vec<i32> = Vec::new();
        for x in (0..len).step_by(2) {
            for y in 0..nums[x] {
                res.push(nums[x+1])
            }
        }
        return res;
    }
}
