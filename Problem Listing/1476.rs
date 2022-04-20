// https://leetcode.com/problems/subrectangle-queries/
// 16 ms, 3.3 MB

struct SubrectangleQueries {
    rectangle: Vec<Vec<i32>>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SubrectangleQueries {

    fn new(rectangle: Vec<Vec<i32>>) -> Self {
        return SubrectangleQueries { rectangle };
    }
    
    fn update_subrectangle(&mut self, row1: i32, col1: i32, row2: i32, col2: i32, new_value: i32) {
        for x in row1..row2+1 {
            for y in col1..col2+1 {
                self.rectangle[x as usize][y as usize] = new_value;
            }
        }
    }
    
    fn get_value(&self, row: i32, col: i32) -> i32 {
        return self.rectangle[row as usize][col as usize];
    }
}

/**
 * Your SubrectangleQueries object will be instantiated and called as such:
 * let obj = SubrectangleQueries::new(rectangle);
 * obj.update_subrectangle(row1, col1, row2, col2, newValue);
 * let ret_2: i32 = obj.get_value(row, col);
 */
