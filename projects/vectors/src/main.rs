use std::collections::HashMap;
fn main() {
    let mut v:Vec<i32> = vec![1,2,3];
    v.push(3);


    let field_name = String::from("Favourite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();

    let third: i32 = v[2];
    for a in v.iter() {
        println!("{}", a);
    }


    map.insert(field_name, field_value);

    let team_names = String::from("Favourite color");
    if let Some(x) = map.get(&team_names) {
        println!("{}", x);
    }
}
