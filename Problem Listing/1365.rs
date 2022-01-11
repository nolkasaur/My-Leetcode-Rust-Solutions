// https://leetcode.com/problems/how-many-numbers-are-smaller-than-the-current-number/

impl Solution {
    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        let mut res: Vec<i32> = Vec::with_capacity(len);
        for x in 0..len {
            let mut count = 0;
            for y in 0..len {
                if x == y { continue; }
                if nums[y] < nums[x] { count +=1; }
            }
            res.push(count);
        }
        return res;
    }
}
