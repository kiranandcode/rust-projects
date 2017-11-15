#[derive(PartialEq, Debug)]
struct Shoe {
    size: i32,
    style: String
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: i32) -> Vec<Shoe> {
    shoes.into_iter()
        .filter(|s| s.size == shoe_size)
        .collect()
}

#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal,") },
        Shoe { size: 10, style: String::from("boot") }
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe { size: 10, style: String::from("sneaker") },
            Shoe { size: 10, style: String::from("boot") }
        ]);
}


struct Counter {
    count: u32
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    let v1 = vec![1,2,3];
    let v1_iter = v1.iter();
    let counter = Counter::new();

    for val in counter {
        println!("Counter: {}", val);
    }
    for val in v1_iter {
        println!("Got: {}", val);
    }
    println!("Hello, world!");
}
