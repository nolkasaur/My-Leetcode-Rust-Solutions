// https://leetcode.com/problems/unique-morse-code-words/
// 0 ms, 2.1 MB

impl Solution {
    pub fn unique_morse_representations(words: Vec<String>) -> i32 {
        let mut morseArr: Vec<String> = Vec::with_capacity(words.len());
        let morseEncodes = [".-","-...","-.-.","-..",".","..-.","--.","....","..",".---","-.-",".-..","--","-.","---",".--.","--.-",".-.","...","-","..-","...-",".--","-..-","-.--","--.."];
        for word in words {
            let mut strn = String::from("");
            for c in word.chars() {
                strn.push_str(morseEncodes[(c as usize) - 97]);
            } 
            morseArr.push(strn);
        }
        morseArr.sort();
        morseArr.dedup();
        return morseArr.len() as i32;
    }
}
