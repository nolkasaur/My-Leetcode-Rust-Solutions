// https://leetcode.com/problems/execution-of-all-suffix-instructions-staying-in-a-grid/
// 12 ms, 2.3 MB

impl Solution {
    pub fn execute_instructions(n: i32, start_pos: Vec<i32>, s: String) -> Vec<i32> {
        let len = s.len();
        let mut res:Vec<i32> = Vec::with_capacity(len);
        for x in 0..s.len() {
            let mut count = 0;
            let mut currPos = (start_pos[0], start_pos[1]);
            for y in s[x..].chars() {
                match y {
                    'L' => {
                        if currPos.1 - 1 >= 0 { count+=1; currPos = (currPos.0, currPos.1 - 1); } else { break; }
                    },
                    'R' => {
                        if currPos.1 + 1 <= (n-1) as i32 { count+=1; currPos = (currPos.0, currPos.1 + 1); } else { break; }
                    },
                    'U' => {
                        if currPos.0 - 1 >= 0 { count+=1; currPos = (currPos.0 - 1, currPos.1); } else { break; }
                    },
                    'D' => {
                        if currPos.0 + 1 <= (n-1) as i32 { count+=1; currPos = (currPos.0 + 1, currPos.1); } else { break; }
                    },
                      _ => ()
                }
            }
            res.push(count);
        }
        return res;
    }
}
