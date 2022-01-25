// https://leetcode.com/problems/uncommon-words-from-two-sentences/
// 0 ms, 2 MB

impl Solution {
    pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
        let mut res:Vec<String> = Vec::new();
        let splt1:Vec<&str> = s1.split(' ').collect();
        let splt2:Vec<&str> = s2.split(' ').collect();
        for elem in &splt1 {
            if !splt2.contains(&elem) {
                if splt1.iter().filter(|&n| *n == *elem).count() == 1 {
                    res.push(elem.to_string());
                }
            }
        }
        for elem in &splt2 {
            if !splt1.contains(&elem) {
                if splt2.iter().filter(|&n| *n == *elem).count() == 1 {
                    res.push(elem.to_string());
                }
            }
        }
        return res;
    }
}
