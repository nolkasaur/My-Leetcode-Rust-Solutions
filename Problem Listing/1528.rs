// https://leetcode.com/problems/shuffle-string/

impl Solution {
    pub fn restore_string(s: String, indices: Vec<i32>) -> String {
        let len = s.len();
        let mut res: Vec<char> = vec![' '; len];
        for x in 0..len {
            res[indices[x] as usize] = s.chars().nth(x).unwrap();
        }
        return res.iter().collect();;
    }
}
