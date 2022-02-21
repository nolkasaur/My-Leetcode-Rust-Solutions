// https://leetcode.com/problems/maximum-ascending-subarray-sum/
// 0 ms, 2.3 MB

impl Solution {
    pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {
        let mut res = nums[0];
        let mut maxRes = nums[0];
        
        for x in 0..nums.len()-1 {
            if nums[x+1] > nums[x] {
                res += nums[x+1];
            } else {
                if res > maxRes {
                    maxRes = res;
                }
                res = nums[x+1];
            }
        }
        
        if res > maxRes {
            maxRes = res;
        }
        return maxRes;
    }
}
