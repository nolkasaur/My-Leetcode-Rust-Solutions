// https://leetcode.com/problems/decode-xored-array/
// 15 ms, 2.2 MB

impl Solution {
    pub fn decode(encoded: Vec<i32>, first: i32) -> Vec<i32> {
        let len = encoded.len();
        let mut res: Vec<i32> = Vec::with_capacity(len+1);
        res.push(first);
        for x in 0..len {
            res.push(res[x] ^ encoded[x]);
        }
        return res;
    }
}
