// https://leetcode.com/problems/truncate-sentence/
// 0 ms, 2.2 MB

impl Solution {
    pub fn truncate_sentence(s: String, k: i32) -> String {
        let mut res = String::from("");
        let mut split = s.split(' ');
        res += split.nth(0).unwrap();
        let mut count = 1;
        for word in split {
            if count == k { break; }
            res += &format!(" {}", word);
            count += 1;
        }
        return res;
    }
}
