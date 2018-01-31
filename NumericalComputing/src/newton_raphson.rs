use expression::Formula;

pub fn newton_raphson_find_root<'a>(f: &'a Formula, start_x : f64, max_iterations: usize) -> f64 {
    let mut iteration_count = 0;
    let f_prime = f.single_variable_derive();

    let mut x = start_x;
    let mut fx = f.single_variable_eval(x);
    let mut f_prime_x = f_prime.single_variable_eval(x);

    x -= fx / f_prime_x;
    fx = f.single_variable_eval(x);
    f_prime_x = f_prime.single_variable_eval(x);


    while fx.abs() > 0.0 && iteration_count < max_iterations {
        iteration_count += 1;

        x -= fx / f_prime_x;
        fx = f.single_variable_eval(x);
        f_prime_x = f_prime.single_variable_eval(x);
   }

   x 

}

pub fn newton_raphson_find_root_manual<F,G>(f : F, f_prime : G, start_x : f64, max_iterations : usize) -> f64 
    where F : Fn(f64) -> f64,
          G : Fn(f64) -> f64 {
    let mut iteration_count = 0;

    let mut x = start_x;
    let mut fx = f(x);
    let mut f_prime_x = f_prime(x);

    x -= fx / f_prime_x;
    fx = f(x);
    f_prime_x = f_prime(x);


    while fx.abs() > 0.0 && iteration_count < max_iterations {
        iteration_count += 1;

        x -= fx / f_prime_x;
        fx = f(x);
        f_prime_x = f_prime(x);
   }

   x 


}