// https://leetcode.com/problems/divide-a-string-into-groups-of-size-k/
// 2 ms, 2.3 MB

impl Solution {
    pub fn divide_string(mut s: String, k: i32, fill: char) -> Vec<String> {
        let len = s.len();
        let mut y = k - (len as i32 % k);
        while y > 0 {
            s.push(fill);
            y-=1;
        }
        let mut res: Vec<String> = Vec::new();
        let mut x = 0;
        while x < len {
            let arrStr = String::from(&s[x..x+k as usize]);
            res.push(arrStr);
            x+=k as usize;
        }
        return res;
    }
}
