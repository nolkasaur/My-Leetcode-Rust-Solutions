// https://leetcode.com/problems/find-triangular-sum-of-an-array/
// 90 ms, 2.2 MB

impl Solution {
    pub fn triangular_sum(mut nums: Vec<i32>) -> i32 {
        while nums.len() > 1 {
            let mut aux: Vec<i32> = Vec::with_capacity(nums.len()-1);
            for x in 0..nums.len()-1 {
                aux.push((nums[x]+nums[x+1])%10);
            }
            nums = aux.clone();
        }
        return nums[0];
    }
}
