pub struct Counter {
    count: u32
}

impl Counter {
    pub fn new() -> Counter {
        return Counter { count: 0};
    }
}


impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 5 {
            return Some(self.count);
        }
        else {
            return None;
        }
    }
}
