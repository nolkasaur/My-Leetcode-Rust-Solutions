// https://leetcode.com/problems/search-insert-position/
// 2 ms, 2.1 MB

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut leftBound: i32 = 0;
        let mut rightBound: i32 = nums.len() as i32 - 1;
        let mut ind = 0;
        while (leftBound >= 0 && rightBound >= 0 && leftBound <= rightBound) {
            let mut index: i32 = leftBound + (rightBound - leftBound)/2;
            if nums[index as usize] == target { return index; }
            else if nums[index as usize] < target { leftBound = index+1; ind = index+1;}
            else { rightBound = index-1; ind = index; }
        }
        if ind >= 0 { return ind }
        else { return 0; }
    }
}
