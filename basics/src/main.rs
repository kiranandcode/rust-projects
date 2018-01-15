use std::collections::HashMap;
mod extra;
use extra::BinaryTree;

type Table = HashMap<String, Vec<String>>;
fn show(table: &Table) {
    for(artist, works) in table {
        println!("Works by {}:", artist);
        for work in works {
            println!(" {}", work);
        }
    }
}
fn smallest(v: &[i32]) -> &i32 /* lifetime elision */ {
    let mut s = &v[0];
    for r in &v[1..] {
        if r < s {
            s = r;
        }
    }
    s
}

fn example() {
    let mut x = 10;
    let r1 = &x;
    let r2 = &x;
    // can not modify x as it is borrowed
    // x += 10;
    // can not borrow x as mutable as it is borrowed
    // let m = &mut x;
}

fn other_example() {
    let mut v = (107, 109);
    let r = &v;
    let r0 = &r.0; // reborrow okay
    let rr0 = &v.0; // parent path is read only access - can make more refs but not mutable or mutate
}

fn expression_first() {
    let mut i = 0;
    return loop {
        println!("i <- {}", i);
        i = i + 1;
            if i > 3 {
                break;
            }
    };
}

fn divergent_function() -> ! {
    loop {
        println!("Looping forever");
    }
}

fn pattern_matching() {
    let string = match "a string" {
        "a string" => "str",
        other      => other
    };
}


fn main() {
    // rust has several integer types
    let mut v:u8 = 10;
    // each corresponding to the no.of bits
    let mut v:u64 = 20;
    // also supports java-esque sepearators
    let v:i32 = 1_2_3;

    let mut v:i32 = 2;
    // you can also call functions on values
    println!("Exp {}", v.pow(3));
    println!("bool {}", false as i32);
    let arr : [i32;3] = [1,2,3];
    println!("Arr: {:?}", arr);

    println!("Value {}", v);
    println!("Hello, world!");

    println!("Smallest {} ", smallest(&arr));

    let mut tree = BinaryTree::new();
    tree.add("Mercury");
    tree.add("Venus");

}

struct Sieve {
    sieve: [bool;10000]
}

impl Sieve {
    pub fn new() -> Self {
        let mut sieve = Sieve {
            sieve: [true; 10000]
        };

        for i in 2..100 {
            if sieve.sieve[i] {
                let mut j = i * i;
                while j < 10000 {
                    sieve.sieve[j] = false;
                    j += i;
                }
            }
        }
        sieve
    }

    pub fn is_prime(&self, i: usize) -> bool {self.sieve[i]}
}

#[cfg(test)]
mod test {
    use super::Sieve;

    #[test]
    fn sieve_basics() {
        let sieve = Sieve::new();
        assert!(sieve.is_prime(211));
        assert!(!sieve.is_prime(9876));
    }
}