// https://leetcode.com/problems/build-an-array-with-stack-operations/
// 0 ms, 2.2 MB

impl Solution {
    pub fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
        let mut res: Vec<String> = Vec::new();
        let mut num = 1;
        for x in target {
            while num != x {
                res.push(String::from("Push"));
                res.push(String::from("Pop"));
                num += 1;
            }
            res.push(String::from("Push"));
            num += 1;
        }
        return res;
    }
}
