// https://leetcode.com/problems/my-calendar-i/
// 55 ms, 2.8 MB

struct MyCalendar {
    bookings: Vec<i32>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendar {

    fn new() -> Self {
        return MyCalendar { bookings: Vec::new() } 
    }
    
    fn book(&mut self, start: i32, end: i32) -> bool {
        let mut x = 0;
        while x < self.bookings.len() {
            if (start >= self.bookings[x] && start < self.bookings[x+1]) || (end > self.bookings[x] && end <= self.bookings[x+1]) || (start <= self.bookings[x] && end >= self.bookings[x+1]) {
                return false;
            }
            x += 2;
        }
        self.bookings.push(start);
        self.bookings.push(end);
        return true;
    }
}

/**
 * Your MyCalendar object will be instantiated and called as such:
 * let obj = MyCalendar::new();
 * let ret_1: bool = obj.book(start, end);
 */
