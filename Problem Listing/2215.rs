// https://leetcode.com/problems/find-the-difference-of-two-arrays/
// 57 ms, 2.1 MB

impl Solution {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = Vec::new();
        let mut resA: Vec<i32> = Vec::new();
        let mut resB: Vec<i32> = Vec::new();
        for x in 0..nums1.len() {
            if !nums2.contains(&(nums1[x] as i32)) {
                resA.push(nums1[x] as i32);
            }
        }
        for x in 0..nums2.len() {
            if !nums1.contains(&(nums2[x] as i32)) {
                resB.push(nums2[x] as i32);
            }
        }
        resA.sort();
        resA.dedup();
        resB.sort();
        resB.dedup();
        res.push(resA);
        res.push(resB);
        return res;
    }
}
