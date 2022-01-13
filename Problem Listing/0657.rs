// https://leetcode.com/problems/robot-return-to-origin/
// 0 ms, 2.1 MB

impl Solution {
    pub fn judge_circle(moves: String) -> bool {
        let mut pos = [0, 0];
        for x in moves.chars() {
            match x {
                'U' => pos[0]+=1,
                'D' => pos[0]-=1,
                'L' => pos[1]-=1,
                'R' => pos[1]+=1,
                _ => (),
            }
        }
        return pos[0]==0 && pos[1]==0;
    }
}
