impl Solution {
    pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
        let it = operations.iter();
        let mut count = 0;
        for val in it {
            if val.starts_with("+") | val.ends_with("+") {
                count+=1;
            } else {
                count-=1;
            }
        }
        return count;
    }
}
