fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("The value of a[0] is {}", a[0]);
    println!("The returned value is {}",  alternate_function(3));
}


fn alternate_function(x: i32) -> i32 {
    println!("This is another function - doesn't return anything");
    println!("The input parameter is {}", x);
    return x + 1;
}
