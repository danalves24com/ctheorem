

// fn f_prime(x : &f64) -> f64 { 2.0 * x ** &4.0 }
fn f(x : &f64) -> f64 { x**&5.0 / 2.5  + 10.0 }
// fn oracle(x : &f64) -> f64 { 0.7 * x }
fn f_alt(x : &f64) -> f64 { x**&5.0 / 2.5 }

fn main() {
    let a : f64 = 2.0;
    let b : f64 = 4.0;
    let mut count : f64 = 0.0;

    let mut yd : Vec<(f64, f64)> = Vec::new();

    loop {
        if count >= (&b-&a).into() {
            break;
        };

        let x : f64 = &a+&count;
        let q =  {
            (f(&x) - f_alt(&x)) / x
        };

        yd.push((x,q));
        count += 0.1;
    };

    // polynomial fit the quotioent vector



    let mut prev : f64 = yd[0].1;
    for (x,y) in yd.clone() {
        let change = &prev - y;
        print!("{},",change);
        prev = y;
    }
    println!();
    for (x,y) in yd {
        print!("{},",x);
    }
    // Potential relationship
    // log
    // \delta in y_n = sqrt(y_{n-1}) / nthroot



}
