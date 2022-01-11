// https://leetcode.com/problems/number-of-good-pairs/
// 0 ms, 2 MB

impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut count = 0;
        for x in 0..len {
            for y in x+1..len {
                if nums[x] == nums[y] {
                    count +=1;
                }
            }
        }
        return count;
    }
}
