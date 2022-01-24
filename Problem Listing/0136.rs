// https://leetcode.com/problems/single-number/
// 4 ms, 2.4 MB

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut sorted = nums;
        sorted.sort();
        let mut x = 0;
        while x < sorted.len()-1 {
            if sorted[x] != sorted[x+1] { return sorted[x] }
            else { x+=1 }
            x+=1;
        }
        return sorted[sorted.len()-1];
    }
}
