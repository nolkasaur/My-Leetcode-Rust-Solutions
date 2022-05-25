// https://leetcode.com/problems/rotate-array/
// 14 ms, 3.5 MB
// toDo: solution without extra memory

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, mut k: i32) {
        k = k%nums.len() as i32;
        let mut res: Vec<i32> = Vec::with_capacity(nums.len());
        for n in (nums.len() - k as usize)..nums.len() {
            res.push(nums[n]);
        }
        for n in 0..(nums.len() - k as usize) {
            res.push(nums[n]);
        }
        *nums = res;
    }
}
