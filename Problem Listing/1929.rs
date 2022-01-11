// https://leetcode.com/problems/concatenation-of-array/

impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        let mut res: Vec<i32> = Vec::with_capacity(len*2);
        res.extend(&nums);
        res.extend(&nums);
        return res;
    }
}
