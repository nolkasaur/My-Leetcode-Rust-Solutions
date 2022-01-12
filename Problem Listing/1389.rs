// https://leetcode.com/problems/create-target-array-in-the-given-order/
// 0 ms, 2 MB

impl Solution {
    pub fn create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        let mut res: Vec<i32> = Vec::with_capacity(len);
        for x in 0..len {
            let (n, i) = (nums[x], index[x]);
            if i >= res.len() as i32 {
                res.push(n);
            } else {
                res.insert(i as usize, n);
            }
        }
        return res;
    }
}
