// https://leetcode.com/problems/rings-and-rods/
// 0 ms, 2 MB

impl Solution {
    pub fn count_points(rings: String) -> i32 {
        let charArr: Vec<char> = rings.chars().collect();
        let mut res = 0;
        for x in 0..10 {
            let mut red = false;
            let mut green = false;
            let mut blue = false;
            for y in (0..rings.len()-1).step_by(2) {
                if charArr[y+1].to_digit(10).unwrap() == x {
                    if charArr[y] == 'R' {
                        red = true;
                    } else if charArr[y] == 'G' {
                        green = true;
                    } else {
                        blue = true;
                    }
                }
            }
            if red && green && blue {
                res += 1;
            }
        }
        return res;
    }
}
