// https://leetcode.com/problems/longest-continuous-increasing-subsequence/
// 3 ms, 2.2 MB

impl Solution {
    pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
        let mut maxCnt = 1;
        let mut cnt = 1;
        let mut x = 0;
        while x < nums.len()-1 {
            if nums[x+1] > nums[x] {
                cnt += 1;
            } else {
                if cnt > maxCnt { maxCnt = cnt; }
                cnt = 1;
            }
            x += 1;
        }
        if cnt > maxCnt { maxCnt = cnt; }
        return maxCnt;
    }
}
