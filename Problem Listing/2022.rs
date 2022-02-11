// https://leetcode.com/problems/convert-1d-array-into-2d-array/
// 77 ms, 4.5 MB

impl Solution {
    pub fn construct2_d_array(original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {
        if m * n != original.len() as i32 { return [].to_vec() }
        let mut res: Vec<Vec<i32>> = Vec::new();
        let mut count = 0;
        for row in 0..m {
            let mut arr: Vec<i32> = Vec::new();
            for column in 0..n {
                arr.push(original[count]);
                count += 1;
            }
            res.push(arr);
        }
        return res;
    }
}
