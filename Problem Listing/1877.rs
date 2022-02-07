// https://leetcode.com/problems/minimize-maximum-pair-sum-in-array/
// 93 ms, 3.2 MB

impl Solution {
    pub fn min_pair_sum(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let mut res = 0;
        for x in 0..nums.len()/2 {
            let sum = nums[x] + nums[nums.len()-1-x];
            if sum > res {
                res = sum;
            }
        }
        return res;
    }
}
