// https://leetcode.com/problems/rearrange-array-elements-by-sign/
// 90 ms, 6.1 MB

impl Solution {
    pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
        let mut negVec:Vec<i32> = Vec::with_capacity(nums.len()/2);
        let mut posVec:Vec<i32> = Vec::with_capacity(nums.len()/2);
        let mut finalVec:Vec<i32> = Vec::with_capacity(nums.len());
        for num in &nums {
            if *num > 0 {
                posVec.push(*num);
            } else {
                negVec.push(*num);
            }
        }
        for x in 0..nums.len()/2 {
            finalVec.push(posVec[x]);
            finalVec.push(negVec[x]);
        }
        return finalVec;
    }
}
