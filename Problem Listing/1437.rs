// https://leetcode.com/problems/check-if-all-1s-are-at-least-length-k-places-away/
// 8 ms, 2.7 MB

impl Solution {
    pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
        let mut places = 0;
        let mut found = false;
        for num in nums {
            if found {
                places+=1;
            }
            if num == 1 && !found{
                found = true;
            } else if num == 1 && found {
                if places-1 < k { return false }
                places=0;
            }
        }
        return true
    }
}
