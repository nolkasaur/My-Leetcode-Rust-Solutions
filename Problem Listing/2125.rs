// https://leetcode.com/problems/number-of-laser-beams-in-a-bank/
// 12 ms, 2.8 MB

impl Solution {
    pub fn number_of_beams(bank: Vec<String>) -> i32 {
        let mut nrBeams = 0;
        let mut res = 0;
        for x in bank {
            let c = x.matches('1').count();
            res += nrBeams * c;
            if c != 0 { nrBeams = c };
        }
        return res as i32;
    }
}
