// https://leetcode.com/problems/kth-largest-element-in-an-array/
// 4 ms, 2.2 MB

impl Solution {
    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort();
        return nums[nums.len()-k as usize]
    }
}
