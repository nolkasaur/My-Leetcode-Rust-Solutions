// https://leetcode.com/problems/maximum-product-difference-between-two-pairs/
// 8 ms, 2.2 MB

impl Solution {
    pub fn max_product_difference(nums: Vec<i32>) -> i32 {
        let mut sorted = nums;
        sorted.sort();
        let len = sorted.len();
        return (sorted[len-1] * sorted[len-2]) - (sorted[0] * sorted[1]);
    }
}
