use std::collections::HashMap;


fn main() {
    let mut scores = HashMap::new();
    scores.insert("Blue".to_string(), 10);
    scores.insert("Red".to_string(), 50);


    print_state(&scores);

    // overwriting existing values
    println!("Overwriting blue with new value"); 
    scores.insert("Blue".to_string(), 21);
    print_state(&scores);


    // Check if no value
    println!("Attempting to insert Blue: 12, and Yellow: 23 with checks");
    scores.entry("Blue".to_string()).or_insert(12);
    scores.entry("Yellow".to_string()).or_insert(23);
    print_state(&scores);

    counting_example();


    let vec : Vec<i32> = vec![1,2,3,4];

    println!("The avg of the vector is {}", mean(&vec));
}

fn print_state(scores : &HashMap<String, u32>) {
    for(key,value) in scores {
        println!("{}: {}", key, value);
    }
}


fn counting_example() {
    println!("Example of counting words in a string");

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

}


fn mean(x : &Vec<i32>) -> f64 {
    if x.len() == 0 {
        0.0
    } else {
        let mut count : f64 = 0.0;
        let mut sum : f64 = 0.0;
        for value in x.iter() {
           sum += *value as f64; 
           count += 1.0;
        }

        sum = sum / count;

        sum
    }
}

