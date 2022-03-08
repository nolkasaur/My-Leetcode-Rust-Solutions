// https://leetcode.com/problems/two-out-of-three/
// 0 ms, 2.3 MB

impl Solution {
    pub fn two_out_of_three(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();
        for elem in nums1 {
            if nums2.contains(&elem) {
                res.push(elem);
            }
            if nums3.contains(&elem) {
                res.push(elem);
            }
        }
        for elem in nums2 {
            if nums3.contains(&elem) {
                res.push(elem);
            }
        }
        res.sort();
        res.dedup();
        return res;
    }
}
