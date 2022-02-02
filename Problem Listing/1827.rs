// https://leetcode.com/problems/minimum-operations-to-make-the-array-increasing/
// 3 ms, 2.3 MB

impl Solution {
    pub fn min_operations(mut nums: Vec<i32>) -> i32 {
        let mut res = 0;
        for x in 1..nums.len() {
            if nums[x] <= nums[x-1] {
                let val = nums[x-1] + 1;
                res += val - nums[x];
                nums[x] = val;
            }
        }
        return res;
    }
}
