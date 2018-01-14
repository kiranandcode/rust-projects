use std::collections::HashMap;
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