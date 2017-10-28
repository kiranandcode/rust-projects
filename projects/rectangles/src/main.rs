#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        return self.length * self.width;
    }

    fn can_hold(&self, other : &Rectangle) -> bool {
        return (self.length > other.length && self.width > other.width) || (self.width > other.length && self.length > other.width);
    }
}

fn main() {
    let rect = Rectangle {length: 50, width: 40};
    let rect2 = Rectangle {length: 10, width: 45};

    println!(
        "The area of the rectangle is {} square pixels",
        rect.area()
    );
    if rect.can_hold(&rect2)  {
        println!("Rect {:?} can hold {:?}", rect, rect2);
    }
}

fn area(rect: &Rectangle) -> u32 {
    rect.length * rect.width 
}
