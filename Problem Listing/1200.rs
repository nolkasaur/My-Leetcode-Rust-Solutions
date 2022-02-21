// https://leetcode.com/problems/minimum-absolute-difference/
// 20 ms, 3.2 MB

impl Solution {
    pub fn minimum_abs_difference(mut arr: Vec<i32>) -> Vec<Vec<i32>> {
        arr.sort();
        let mut res: Vec<Vec<i32>> = Vec::new();
        let mut minDiff = arr[0].abs() + arr[arr.len()-1].abs();
        let mut iter = arr.windows(2);
        for x in iter {
            if x[1] - x[0] <= minDiff {
                if x[1] - x[0] < minDiff {
                    minDiff = x[1] - x[0];
                    res = Vec::new();
                }
                let mut subArr: Vec<i32> = Vec::with_capacity(2);
                subArr.push(x[0]);
                subArr.push(x[1]);
                res.push(subArr);
            }
        }
        return res;
    }
}
