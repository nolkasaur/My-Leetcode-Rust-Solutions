// https://leetcode.com/problems/lucky-numbers-in-a-matrix/
// 5 ms, 2.2 MB

impl Solution {
    pub fn lucky_numbers (matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();
        let nrRows = matrix.len();
        let nrCols = matrix[0].len();
        let mut minRowVec: Vec<i32> = Vec::with_capacity(nrRows);
        let mut maxColVec: Vec<i32> = vec![0; nrCols];
        for row in 0..nrRows {
            let mut minRow = 100001;
            for col in 0..nrCols {
                if matrix[row][col] < minRow {
                    minRow = matrix[row][col];
                }
                if matrix[row][col] > maxColVec[col] {
                    maxColVec[col] = matrix[row][col];
                }
            }
            minRowVec.push(minRow);
        }
        for x in &minRowVec {
            for y in &maxColVec {
                if *x == *y {
                    res.push(*x);
                }
            }
        }
        return res;
    }
}
