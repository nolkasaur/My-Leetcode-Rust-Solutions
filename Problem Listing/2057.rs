// https://leetcode.com/problems/smallest-index-with-equal-value/
// 3 ms, 2.1 MB

impl Solution {
    pub fn smallest_equal(nums: Vec<i32>) -> i32 {
        for x in 0..nums.len() {
            if x % 10 == nums[x] as usize { return x as i32; }
        }
        return -1
    }
}
