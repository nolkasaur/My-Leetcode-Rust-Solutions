// https://leetcode.com/problems/occurrences-after-bigram/
// 0 ms, 2.3 MB

impl Solution {
    pub fn find_ocurrences(text: String, first: String, second: String) -> Vec<String> {
        let splt: Vec<&str> = text.split(" ").collect();
        let mut res = Vec::new();
        let mut x = 0;
        while x < splt.len()-2 {
            if splt[x] == first && splt[x+1] == second {
                res.push(String::from(splt[x+2]));
            }
            x+=1;
        }
        return res;
    }
}
