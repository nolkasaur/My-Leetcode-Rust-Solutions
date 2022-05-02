// https://leetcode.com/problems/valid-parentheses/
// 23 ms, 2.1 MB

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut parentheses: Vec<char> = Vec::new();
        for x in 0..s.len() {
            let character = s.chars().nth(x).unwrap();
            if (character == ')' && parentheses.len() > 0 && parentheses[parentheses.len()-1] == '(') ||
               (character == ']' && parentheses.len() > 0 && parentheses[parentheses.len()-1] == '[') ||
               (character == '}' && parentheses.len() > 0 && parentheses[parentheses.len()-1] == '{') {
                   parentheses.pop();
            } else {
                parentheses.push(character);
            }
        }
        return parentheses.is_empty();
    }
}
