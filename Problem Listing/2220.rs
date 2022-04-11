// https://leetcode.com/problems/minimum-bit-flips-to-convert-number/
// 0 ms, 2.1 MB

impl Solution {
    pub fn min_bit_flips(start: i32, goal: i32) -> i32 {
        let mut res = 0;
        let mut startBin = format!("{:b}", start);
        let mut goalBin = format!("{:b}", goal);
        let mut startBinFinal = String::from("");
        let mut goalBinFinal = String::from("");
        if startBin.len() > goalBin.len() {
            let mut str = "0".repeat(startBin.len()-goalBin.len());
            startBinFinal = startBin;
            goalBinFinal = format!("{}{}", str, goalBin);
        } else {
            let mut str = "0".repeat(goalBin.len()-startBin.len());
            goalBinFinal = goalBin;
            startBinFinal = format!("{}{}", str, startBin);
        }
        for x in 0..startBinFinal.len() {
            if startBinFinal.chars().nth(x). unwrap()!=goalBinFinal.chars().nth(x). unwrap() {
                res+=1;
            }
        }
        return res;
    }
}
