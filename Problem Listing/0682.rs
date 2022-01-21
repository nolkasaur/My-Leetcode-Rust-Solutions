// https://leetcode.com/problems/baseball-game/
// 0 ms, 2.2 MB

impl Solution {
    pub fn cal_points(ops: Vec<String>) -> i32 {
        let mut resVec:Vec<i32> = Vec::new();
        let mut res = 0;
        for x in 0..ops.len() {
            let len = resVec.len();
            match &*ops[x] {
                "C" => { resVec.remove(len-1); () },
                "D" => resVec.insert(len, resVec[len-1]*2),
                "+" => resVec.insert(len, resVec[len-1] + resVec[len-2]),
                other => resVec.insert(len, other.parse::<i32>().unwrap()),
            }
        }
        for y in resVec {
            res += y;
        }
        return res;
    }
}
