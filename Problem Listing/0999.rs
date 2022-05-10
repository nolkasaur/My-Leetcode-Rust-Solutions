// https://leetcode.com/problems/available-captures-for-rook/
// 1 ms, 2.2 MB

impl Solution {
    pub fn num_rook_captures(board: Vec<Vec<char>>) -> i32 {
        let mut x = 0;
        let mut y = 0;
        let mut res = 0;
        'outer: for r in 0..8 {
            for c in 0..8 {
                if board[r][c] == 'R' {
                    x = c;
                    y = r;
                    break 'outer;
                }
            }
        }
        for n in x+1..8 {
            if board[y][n] == 'B' {
                break;
            } else if board[y][n] == 'p' {
                res += 1;
                break;
            }
        }
        for n in 1..x+1 {
            if board[y][x-n] == 'B' {
                break;
            } else if board[y][x-n] == 'p' {
                res += 1;
                break;
            }
        }
       for n in y+1..8 {
            if board[n][x] == 'B' {
                break;
            } else if board[n][x] == 'p' {
                res += 1;
                break;
            }
        }
        for n in 1..y+1 {
            if board[y-n][x] == 'B' {
                break;
            } else if board[y-n][x] == 'p' {
                res += 1;
                break;
            }
        }
        return res;
    }
}
