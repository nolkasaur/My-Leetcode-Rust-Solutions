// https://leetcode.com/problems/special-positions-in-a-binary-matrix/
// 5 ms, 2.4 MB

impl Solution {
    pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;
        let mut nrOnesRows: Vec<i32> = Vec::with_capacity(mat.len());
        let mut nrOnesCols: Vec<i32> = vec![0; mat[0].len()];
        for x in 0..mat.len() {
            let mut sumRow = 0;
            for y in 0..mat[0].len() {
                sumRow += mat[x][y];
                nrOnesCols[y] += mat[x][y];
            }
            nrOnesRows.push(sumRow);
        }
        for i in 0..mat.len() {
            for j in 0..mat[0].len() {
                if mat[i][j] == 1 && nrOnesRows[i] == 1 && nrOnesCols[j] == 1 {
                    res += 1;
                }
            }
        }
        return res;
    }
}
