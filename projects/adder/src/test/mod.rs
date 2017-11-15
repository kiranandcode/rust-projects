use super::Rectangle;
use super::add_two;

#[test]
fn larger_can_hold_smaller() {
    let larger = Rectangle { length: 8, width: 7 }; 
    let smaller = Rectangle { length: 5, width: 1};

    assert!(larger.can_hold(&smaller));
}


#[test]
fn smaller_can_not_hold_larger( ){
    let larger = Rectangle { length: 8, width: 7 }; 
    let smaller = Rectangle { length: 5, width: 1};

    assert!(!smaller.can_hold(&larger));
}


#[test]
fn it_adds_two() {
    assert_eq!(4, add_two(2));
}
