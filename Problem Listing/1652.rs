// https://leetcode.com/problems/defuse-the-bomb/
// 2 ms, 2.1 MB

impl Solution {
    pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::with_capacity(code.len());
        if k == 0 {
            return vec![0; code.len()];
        }
        if k > 0 {
            for x in 0..code.len() {
                let mut sum = 0;
                for y in 1..(k+1) as usize {
                    sum += code[(x+y)%code.len()];
                }
                res.push(sum);
            }
        } else {
            for x in 0..code.len() {
                let mut sum = 0;
                for y in 1..(k.abs()+1) as usize {
                    if ((x - y) as i32) < 0 {
                        sum += code[code.len()-y+x];
                    } else {
                        sum += code[(x-y)%code.len()];
                    }
                }
                res.push(sum);
            }
        }
        return res;
    }
}
