// https://leetcode.com/problems/build-array-from-permutation/
// 12 ms, 2.3 MB

impl Solution {
    pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        let mut res: Vec<i32> = Vec::with_capacity(len);
        for x in 0..len {
            res.push(nums[nums[x] as usize]);
        }
        return res;
    }
}
