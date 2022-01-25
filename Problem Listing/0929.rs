// https://leetcode.com/problems/unique-email-addresses/
// 7 ms, 2.2 MB

impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        let mut resVec:Vec<String> = Vec::new();
        for email in emails {
            let mut str = String::from("");
            let splt1:Vec<&str> = email.split('@').collect();
            let word1 = splt1[0];
            let splt2:Vec<&str> = word1.split('+').collect();
            let mut word2 = splt2[0];
            let word2 = &word2.replace(".", "");
            str.push_str(word2);
            str.push('@');
            let splt3:Vec<&str> = email.split('@').collect();
            let word3 = splt3[1];
            str.push_str(word3);
            resVec.push(str);
        }
        resVec.sort();
        resVec.dedup();
        return resVec.len() as i32;
    }
}
