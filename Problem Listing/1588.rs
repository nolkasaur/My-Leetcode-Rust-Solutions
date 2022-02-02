// https://leetcode.com/problems/sum-of-all-odd-length-subarrays/
// 0 ms, 2.1 MB

impl Solution {
    pub fn sum_odd_length_subarrays(arr: Vec<i32>) -> i32 {
        let mut res: i32 = 0;
        let mut x = 1;
        while x <= arr.len() {
            let mut iter = arr.windows(x);
            for elem in iter {
                for num in elem {
                    res += num;
                }
            }
            x += 2
        }
        return res;
    }
}
