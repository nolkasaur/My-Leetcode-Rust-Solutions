// https://leetcode.com/problems/reformat-phone-number/
// 0 ms, 2 MB

impl Solution {
    pub fn reformat_number(number: String) -> String {
        let rm = number.replace(" ", "").replace("-", "");
        let mut res = String::from("");
        let mut count = 0;
        let len = rm.len();
        while count < len {
            if len % 3 == 0 || count + 4 < len {
                res.push(rm.chars().nth(count).unwrap());
                res.push(rm.chars().nth(count+1).unwrap());
                res.push(rm.chars().nth(count+2).unwrap());
                res.push('-');
                count+=3;
            } else {
                res.push(rm.chars().nth(count).unwrap());
                res.push(rm.chars().nth(count+1).unwrap());
                res.push('-');
                count+=2;
            }
            println!("{}", res);
        }
        return res[..res.len()-1].to_string();
    }
}
