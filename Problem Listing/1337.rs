// https://leetcode.com/problems/the-k-weakest-rows-in-a-matrix/
// 5 ms, 2.2 MB

impl Solution {
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let len: i32 = mat.len() as i32;
        let mut res:Vec<i32> = Vec::with_capacity(len as usize);
        'outer: for x in 0..mat[0].len() as i32 {
            'inner: for y in 0..len {
                if res.contains(&(len-1-y)) {
                    continue 'inner;
                }
                if mat[(len-1-y) as usize][mat[0].len()-1-x as usize] == 1 {
                    res.push(len-1-y);
                }
            }
        }
        for z in 0..len {
            if mat[(len-1-z) as usize][0] == 0 {
                res.push(len-1-z);
            }
        }
        if res.len() == 0 {
            res.push(0);
        } else {
            res.reverse();
        }
        return res[0..k as usize].to_vec();
    }
}
