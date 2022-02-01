// https://leetcode.com/problems/maximum-product-of-three-numbers/
// 19 ms, 2.4 MB

use std::cmp::max;

impl Solution {
    pub fn maximum_product(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let a = nums[0]*nums[1]*nums[2];
        let b = nums[0]*nums[1]*nums[nums.len()-1];
        let c = nums[0]*nums[nums.len()-2]*nums[nums.len()-1];
        let d = nums[nums.len()-3]*nums[nums.len()-2]*nums[nums.len()-1];
        let mut maxN = max(a, b);
        maxN = max(maxN, c);
        maxN = max(maxN, d);
        return maxN;
    }
}
