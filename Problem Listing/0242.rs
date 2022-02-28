// https://leetcode.com/problems/valid-anagram/
// 8 ms, 2.6 MB

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut charsA: Vec<char> = s.chars().collect();
        let mut charsB: Vec<char> = t.chars().collect();
        charsA.sort_by(|a, b| b.cmp(a));
        charsB.sort_by(|a, b| b.cmp(a));
        return charsA == charsB;
    }
}
