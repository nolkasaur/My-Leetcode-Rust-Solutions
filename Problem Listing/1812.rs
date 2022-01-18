// https://leetcode.com/problems/determine-color-of-a-chessboard-square/
// 0 ms, 2 MB

impl Solution {
    pub fn square_is_white(coordinates: String) -> bool {
        let x = match coordinates.chars().nth(0).unwrap() {
            'a' => 1,
            'b' => 2,
            'c' => 3,
            'd' => 4,
            'e' => 5,
            'f' => 6,
            'g' => 7,
            'h' => 8,
              _ => 0
        };
        let y = coordinates.chars().nth(1).unwrap().to_digit(10).unwrap();
        return (x + y) % 2 != 0;
    }
}
