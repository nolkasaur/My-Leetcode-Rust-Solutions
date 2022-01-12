// https://leetcode.com/problems/count-items-matching-a-rule/
// 16 ms, 4.1 MB

impl Solution {
    pub fn count_matches(items: Vec<Vec<String>>, rule_key: String, rule_value: String) -> i32 {
        let mut res = 0;
        let key = match rule_key.as_str() {
            "type" => 0,
            "color" => 1,
            "name" => 2,
            _ => 0
        };
        for x in 0..items.len() {
            if items[x][key] == rule_value { res+=1; }
        }
        return res;
    }
}
