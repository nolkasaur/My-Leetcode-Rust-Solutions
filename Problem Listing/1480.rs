// https://leetcode.com/problems/running-sum-of-1d-array/
// 3 ms, 2.3 MB

impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut count = 0;
        let mut res: Vec<i32> = Vec::new();
        let it = nums.iter();
        for val in it {
            res.push(count+val);
            count+=val;
        }
        return res;
    }
}
