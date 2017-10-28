use std::io;
fn main() {
    let mut x = String::new();
    loop {
        println!("What would you like to do?\n\t c - convert temperatures between farenheit and celcius\n\t f - generate the nth fibonacci number\n\t s - print a song");

        io::stdin().read_line(&mut x).expect("Failed to read line");

        match x.as_ref() {
            "c\n" => {
                temp_conv();
                break;
            },
            "f\n" => {
                fibonacci();
                break;
            },
            "s\n" => {
                song();
                break;
            },
            _ => println!("Sorry, that wasn't a valid option\n")
        }
    }
}

fn temp_conv() {
    loop {
        let mut x: String = String::new();
        let mut reference = "";
        println!("Would you like to convert from [f]arenheit or [c]elcius?");
        io::stdin().read_line(&mut x).expect("Failed to read line");

        match x.as_ref() {
            "c\n" => {
                reference = "celcius";
            }
            "f\n" => {
                reference = "farenheit";
            }
            _     => {
                println!("Sorry, that isn't a valid option");
                continue;
            }
        }

        println!("Please enter a value in {}:", reference);
        io::stdin().read_line(&mut x).expect("Failed to read line");

        let u: f64 = match x.trim().parse() {
            Ok(num) => num,
            Err(e)  => {
                println!("Error occurred");
                continue;
            }
        };
        
        match reference {
            "celcius" => {
                let result = (u * (5.0/9.0)) + 32.0;
                println!("{} celcius is {} farenheit", u, result);
                break;
            },
            "farenheit" => {
                let result = (u - 32.0) * (5.0/9.0);
                println!("{} farenheit is {} celcius", u, result);
                break;
            }
            _ => {
                break;
            }
        }
    }
}

fn fibonacci() {
    let mut inp = String::new();
    let mut n:u32 = 0;
    let mut i = 0;
    let mut a = 0;
    let mut b = 1;

    println!("Which one of the fibonacci numbers would you like?");
    io::stdin().read_line(&mut inp).expect("Failed to read line");
    
    n = match inp.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error");
            0
        }
    };



    while i < n {
        let c = a + b;
        a = b;
        b = c;
        i = i + 1;
    }
    println!("The {}th fibonacci number is {}", i, b);
}


fn song() {
    println!("This is a song - You wasted a choice by selecting this. Good job");

}
