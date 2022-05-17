// https://leetcode.com/problems/binary-search/
// 6 ms, 2.2 MB

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut leftBound: i32 = 0;
        let mut rightBound: i32 = nums.len() as i32 -1;
        while (leftBound >= 0 && rightBound >= 0 && leftBound <= rightBound) {
            let mut index: i32 = leftBound + (rightBound - leftBound)/2;
            if nums[index as usize] == target { return index; }
            else if nums[index as usize] < target { leftBound = index+1; }
            else {
                rightBound = index-1;
            }
        }
        return -1;
    }
}
