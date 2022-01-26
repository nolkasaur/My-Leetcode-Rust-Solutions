// https://leetcode.com/problems/majority-element/
// 8 ms, 2.3 MB

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut sorted = nums;
        sorted.sort();
        let mut count = 1;
        let mut maxCount = 1;
        let mut res = sorted[0];
        let mut maxRes = sorted[0];
        for x in 1..sorted.len() {
            if sorted[x] == sorted[x-1] {
                count+=1;
            } else {
                if count > maxCount {
                    maxCount = count;
                    maxRes = sorted[x-1];
                }
                res = sorted[x];
                count = 1;
            }
        }
        if count > maxCount {
            maxCount = count;
            maxRes = sorted[sorted.len()-1];
        }
        return maxRes;
    }
}
