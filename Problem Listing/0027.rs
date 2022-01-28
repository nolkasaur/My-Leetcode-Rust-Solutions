// https://leetcode.com/problems/remove-element/
// 4 ms, 2 MB

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut count = 0;
        let mut x = 0;
        while x < nums.len() {
            if nums[x] == val {
                nums.remove(x);
            } else {
                count += 1;
                x += 1;
            }
        }
        return count;
    }
}
