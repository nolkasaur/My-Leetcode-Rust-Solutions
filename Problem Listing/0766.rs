// https://leetcode.com/problems/toeplitz-matrix/
// 6 ms, 2.2 MB

impl Solution {
    pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
        for x in 0..matrix.len()-1 {
            for y in 0..matrix[0].len()-1 {
                if matrix[x][y] != matrix[x+1][y+1] {
                    return false;
                }
            }
        }
        return true;
    }
}
