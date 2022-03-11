// https://leetcode.com/problems/third-maximum-number/
// 0 ms, 2.2 MB

impl Solution {
    pub fn third_max(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        nums.dedup();
        if nums.len() < 3 {
            return nums[nums.len()-1];
        } else {
            return nums[nums.len()-3];
        }
    }
}
