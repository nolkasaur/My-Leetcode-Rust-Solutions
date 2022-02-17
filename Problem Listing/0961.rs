// https://leetcode.com/problems/n-repeated-element-in-size-2n-array/
// 14 ms, 2.2 MB

impl Solution {
    pub fn repeated_n_times(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        for x in 0..nums.len() {
            if nums[x] == nums[x+1] {
                return nums[x];
            }
        }
        return -1;
    }
}
