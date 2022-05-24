// https://leetcode.com/problems/squares-of-a-sorted-array/
// 19 ms, 2.2 MB

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::with_capacity(nums.len());
        for n in nums {
            res.push(n*n);
        }
        res.sort();
        return res;
    }
}
