// https://leetcode.com/problems/sorting-the-sentence/
// 1 ms, 2.1 MB

impl Solution {
    pub fn sort_sentence(s: String) -> String {
        let mut split = s.split(' ');
        let mut strVec = vec![" "; s.matches(' ').count()+1];
        for x in split {
            let ind = x.chars().last().unwrap().to_digit(10).unwrap()-1;
            strVec[ind as usize] = &x[..x.len()-1];
        }
        return strVec.join(" ");
    }
}
