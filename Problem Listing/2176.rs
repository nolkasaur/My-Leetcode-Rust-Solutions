// https://leetcode.com/problems/count-equal-and-divisible-pairs-in-an-array/
// 0 ms, 2.1 MB

impl Solution {
    pub fn count_pairs(nums: Vec<i32>, k: i32) -> i32 {
        let mut res = 0;
        for x in 0..nums.len()-1 {
            for y in x+1..nums.len() {
                if nums[x] == nums[y] && (x*y) as i32 % k == 0 {
                    res += 1;
                }
            }
        }
        return res;
    }
}
