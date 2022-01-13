// https://leetcode.com/problems/matrix-diagonal-sum/
// 0 ms, 2.1 MB

impl Solution {
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        let len = mat[0].len();
        let skip = len % 2 != 0;
        let skipNum = len / 2;
        let mut res = 0;
        for x in 0..len {
            res+=mat[x][x];
            if !(skip && x==skipNum) { res+=mat[x][len-x-1]; }
        }
        return res;
    }
}
