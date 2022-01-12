// https://leetcode.com/problems/count-number-of-pairs-with-absolute-difference-k/
// 0 ms, 1.9 MB

impl Solution {
    pub fn count_k_difference(nums: Vec<i32>, k: i32) -> i32 {
        let len = nums.len();
        let mut res = 0;
        for x in 0..len {
            for y in x..len {
                if (nums[x]-nums[y]).abs() == k { res+=1; }
            }
        }
        return res;
    }
}
