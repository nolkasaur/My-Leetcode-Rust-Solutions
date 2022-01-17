// https://leetcode.com/problems/determine-if-string-halves-are-alike/
// 0 ms, 2.3 MB

impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        let mut countA = 0;
        let mut countB = 0;
        for x in s[0..s.len()/2].chars() {
            match x {
                'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => countA+=1,
                _ => (),
            }
        }
        for y in s[s.len()/2..s.len()].chars() {
            match y {
                'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => countB+=1,
                _ => (),
            }
        }
        return countA == countB;
    }
}
