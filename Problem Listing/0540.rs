// https://leetcode.com/problems/single-element-in-a-sorted-array/
// 5 ms, 2.9 MB

impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        if nums.len() < 2 { return nums[0] }
        let mut x = 0;
        while x < nums.len()-1 {
            if nums[x] != nums [x+1] { return nums[x] }
            x += 2;
        }
        return nums[nums.len()-1];
    }
}
