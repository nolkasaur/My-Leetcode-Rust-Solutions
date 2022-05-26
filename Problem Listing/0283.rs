// https://leetcode.com/problems/move-zeroes/
// 12 ms, 2.2 MB

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut n = 0;
        let mut m = 0;
        while m < nums.len() {
            if nums[n] == 0 {
                nums.push(nums[n]);
                nums.remove(n);
            } else {
                n += 1;
            }
            m += 1;
        }
    }
}
