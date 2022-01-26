// https://leetcode.com/problems/capitalize-the-title/
// 0 ms, 2.3 MB

impl Solution {
    pub fn capitalize_title(title: String) -> String {
        let mut capTitle = title.to_lowercase();
        let mut capTitleVec: Vec<&str> = capTitle.split(' ').collect();
        let mut capTitleVecString: Vec<String> = Vec::with_capacity(capTitleVec.len());
        for x in 0..capTitleVec.len() {
            let word = capTitleVec[x];
            if word.len() > 2 {
                let mut capWord = String::from("");
                let mut capChar = word.chars().nth(0).unwrap().to_string();
                capChar = capChar.to_uppercase();
                capWord.push_str(&capChar);
                capWord.push_str(&word[1..]);
                capTitleVecString.push(String::from(capWord));
            } else {
                capTitleVecString.push(String::from(word));
            }
        }
        return capTitleVecString.join(" ");
    }
}
