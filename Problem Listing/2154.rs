// https://leetcode.com/problems/keep-multiplying-found-values-by-two/
// 0 ms, 2.1 MB

impl Solution {
    pub fn find_final_value(nums: Vec<i32>, original: i32) -> i32 {
        let mut res = original;
        while nums.contains(&res) {
            res *= 2;
        }
        return res;
    }
}
