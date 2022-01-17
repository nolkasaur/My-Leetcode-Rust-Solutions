// https://leetcode.com/problems/truncate-sentence/
// 3 ms, 2 MB

impl Solution {
    pub fn truncate_sentence(s: String, k: i32) -> String {
        let mut res = String::from("");
        let mut split: Vec<&str> = s.split(' ').collect();
        res += split[0];
        for x in 1..k {
            res += &format!(" {}", split[x as usize]);
        }
        return res;
    }
}
