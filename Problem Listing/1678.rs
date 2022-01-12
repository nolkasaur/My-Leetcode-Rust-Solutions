// https://leetcode.com/problems/goal-parser-interpretation/
// 0 ms, 2 MB

impl Solution {
    pub fn interpret(command: String) -> String {
        let mut res = String::new();
        let mut index = 0;
        while index < command.len() {
            let c = command.chars().nth(index).unwrap();
            match c {
                'G' => {
                            res.push('G');
                            index+=1
                        },
                ')' => {
                            res.push('o');
                            index+=1
                        },
                'a' => {
                            res.push_str("al");
                            index+=3
                        },
                  _ => {
                            index+=1;
                            continue
                        },
            }
        }
        return res;
    }
}
