// https://leetcode.com/problems/sign-of-the-product-of-an-array/
// 0 ms, 2.1 MB

impl Solution {
    pub fn array_sign(nums: Vec<i32>) -> i32 {
        let mut negatives = 0;
        for num in nums {
            if num < 0 { negatives += 1; }
            else if num == 0 { return 0 }
        }
        if negatives % 2 == 0 { return 1 }
        else { return -1 }
    }
}
