// https://leetcode.com/problems/mean-of-array-after-removing-some-elements/
// 0 ms, 2.3 MB

impl Solution {
    pub fn trim_mean(mut arr: Vec<i32>) -> f64 {
        arr.sort();
        let len = arr.len() as f64;
        let mut sum = 0.0;
        let gap = 0.05 * len;
        let mut c = gap;
        while c < len - gap {
            sum += arr[c as usize] as f64;
            c += 1.0;
        }
        return sum / (len - (gap * 2.0));
    }
}
