use std::io;
use std::io::prelude::*;
use array::DimensionArray3;



pub fn run() {
    // basic boilerplate
    let mut input = io::stdin();
    let mut string = String::new();

    // read in the input
    input.read_line(&mut string).unwrap();

    let elems = string.split(" ").map(|seq| seq.trim().parse::<u32>().unwrap()).collect::<Vec<_>>();

    // sweet and sexy
    let (n,k) = (elems[0], elems[1]);



    // now we want to find the number of triples (a,b,c) of positive integers lte N,
    // such that a + b, b + c, c + a are multiples of K

    // thus

    //  q_ab * k = a + b
    //  q_bc * k = b + c
    //  q_ca * k = c + a


    // this means that no pair of triples will be greater than k, and thus cannot
    // be multiples
    if 2 * n < k {
        println!("0");
    }


    // now we will use dynamic programming to solve the problem

    let n = n + 1;
    let mut array = DimensionArray3::new((n as usize,n as usize,n as usize), 0);

    // optimal substructure:
    // f (a,b,c) = # no_triples of of integers lte (a,b,c)
    // k (a,b,c) = (a + b)%k == 0 && (b + c)%k == 0 && (c + a)%k == 0 ? 1 : 0

    //  f (i,j,k) =
    //     f (p_i, j, k) +
    //     f (i, p_j, k) +
    //     f (i, j, p_k) +
    //     k(i,j,k)


    // 3 2
    // f (1,1,1)  =
    //    f(0,1,1) +
    //    f(1,0,1) +
    //    f(1,1,0) +
    //    1

    // f (1,1,3) =
    //    f(0,1,3) +
    //    f(1,0,3) +
    //    f(1,0,2) +
    //    1

    for h in 1..n {
        for i in 1..n {
            for j in 1..n {
                let v =  ((h + i)%k == 0 && (i + j)%k == 0 && (j + h)%k == 0);

                    let t1 = array[((h - 1) as usize, i as usize, j as usize)];
                    let t2 = array[(h as usize, (i - 1) as usize, j as usize)];
                    let t3 = array[(h as usize, i as usize, (j - 1) as usize)];

                if v {
                    println!("{:?} satisfies with priors {:?} => {:?}", (h,i,j), (t1,t2,t3), ((v as u32) + t1 + t2 + t3));
                }
                let v = v as u32;


                    array[(h as usize,i as usize,j as usize)] = v + t1 + t2 + t3;
            }
        }
    }
    println!("{}", array[((n-1) as usize, (n-1) as usize, (n-1) as usize)])

}
