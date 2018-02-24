
use expression::Formula;

pub fn secant_find_root<'a>(formula : &'a Formula, start_x : f64, initial_step_size : f64, max_iterations : usize) -> f64 {
    let mut iteration_count = 0;


    let mut x_2 = start_x - initial_step_size;
    let mut x_1 = start_x;
    let mut fx_2 = formula.single_variable_eval(x_2);
    let mut fx_1 = formula.single_variable_eval(x_1);

    let mut x = x_1 - fx_1 * (x_1 - x_2) / (fx_1 - fx_2);
    let mut fx = formula.single_variable_eval(x);


    while fx.abs() > 0.0 && iteration_count < max_iterations {
        iteration_count += 1;

        x_2 = x_1;
        fx_2 = fx_1;
        x_1 = x;
        fx_1 = fx;

        x = x_1 - fx_1 * (x_1 - x_2) / (fx_1 - fx_2);
        fx = formula.single_variable_eval(x);
   }

   x 
}

pub fn secant_find_root_manual<F>(f : F, start_x : f64, initial_step_size : f64, max_iterations : usize, list : &mut Vec<(f64, f64, f64)>,  iter_freq : usize) -> f64 
    where F : Fn(f64) -> f64 {
     let mut iteration_count = 0;


    let mut x_2 = start_x - initial_step_size;
    let mut x_1 = start_x;
    let mut fx_2 = f(x_2);
    let mut fx_1 = f(x_1);

    let mut x = x_1 - fx_1 * (x_1 - x_2) / (fx_1 - fx_2);
    let mut fx = f(x);


    while fx.abs() > 0.0 && iteration_count < max_iterations {
        iteration_count += 1;

        if iteration_count % iter_freq == 0 {
            list.push((iteration_count as f64, x, fx));
        }

        x_2 = x_1;
        fx_2 = fx_1;
        x_1 = x;
        fx_1 = fx;

        x = x_1 - fx_1 * (x_1 - x_2) / (fx_1 - fx_2);
        fx = f(x);
   }

            list.push((iteration_count as f64, x, fx));
   x 
   
}