// https://leetcode.com/problems/array-partition-i/
// 23 ms, 2.4 MB

impl Solution {
    pub fn array_pair_sum(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let mut res = 0;
        let mut x = 0;
        while x < nums.len() {
            res += nums[x];
            x += 2;
        }
        return res;
    }
}
