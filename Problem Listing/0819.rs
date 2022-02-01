// https://leetcode.com/problems/most-common-word/
// 3 ms, 2.3 MB

use std::collections::HashMap;

impl Solution {
    pub fn most_common_word(paragraph: String, banned: Vec<String>) -> String {
        let mut wordCounter: HashMap<String, u32> = HashMap::new();
        let strippedString = paragraph.replace("!", " ").replace("?", " ").replace("'", " ").replace(",", " ").replace(";", " ").replace(".", " ").replace("  ", " ").to_lowercase();
        let wordVec = strippedString.trim().split(' ');
        for word in wordVec {
            let count = wordCounter.entry(word.to_string()).or_insert(0);
            *count += 1;
        }
        let mut maxCount = 0;
        let mut maxWord = String::from("");
        for (k, v) in wordCounter.iter() {
            if !banned.contains(k) && v > &maxCount {
                maxCount = *v;
                maxWord = k.to_string();
            }
        }
        return maxWord;
    }
}
