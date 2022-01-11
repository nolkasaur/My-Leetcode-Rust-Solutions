// https://leetcode.com/problems/shuffle-the-array/

impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::with_capacity((n*2) as usize);
        for x in 0..n {
            res.push(nums[x as usize]);
            res.push(nums[(x+n) as usize]);
        }
        return res;
    }
}
