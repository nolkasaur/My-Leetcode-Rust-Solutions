// https://leetcode.com/problems/single-number-iii/
// 3 ms, 2.4 MB

impl Solution {
    pub fn single_number(mut nums: Vec<i32>) -> Vec<i32> {
        let mut res = Vec::with_capacity(2);
        nums.sort();
        let mut n = 0;
        while n < nums.len()-1 {
            if nums[n]!=nums[n+1] {
                res.push(nums[n]);
                n+=1;
            } else {
                n+=2;
            }
        }
        if nums[nums.len()-1] != nums[nums.len()-2] {
            res.push(nums[nums.len()-1]);
        }
        return res;
    }
}
