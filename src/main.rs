// import time
// 
// 
// def calculate_pi(n: int) -> float:
//     pi = 0.0
//     div = 1.0
//     mult = 1.0
//     for _ in range(n):
//         pi += (4.0 / div) * mult
//         div += 2.0
//         mult *= -1.0
//     return pi
// 
// 
// if __name__ == '__main__':
//     start = time.time()
//     n = 10
//     for _ in range(8):
//         pi = calculate_pi(n)
//         print(f"{n:12} ... {pi}")
//         n *= 10
//     stop = time.time()
// 
//     print("XXXXXXXXXXXX ... 3.14159265358979323846264338327950288419716939937510")
//     print(f"done in {(stop - start)}s")

use std::time::{Instant};

fn main() {
    println!("Calculate Pi");

    let start = Instant::now();

    let correct:f64 = 3.14159265358979323846264338327950288419716939937510;
    let mut n:  i64 = 1; 

    for i in 1 ..= 8 {
        n = n * 10;
        let pi: f64 = calculate_pi(n);
        let delta = (correct - pi).abs();
        println!("{i:2} calculate pi({n:>#11}) = {pi:<20} delta = {delta:?}")
    }

    let duration = start.elapsed();

    println!("Time elapsed in calculate_pi() is: {:#?}", duration);
   
}

fn calculate_pi(n: i64) -> f64
{
    let mut pi:   f64 = 0.0;
    let mut div:  f64 = 1.0;
    let mut mult: f64 = 1.0;

    for _ in 1..=n {
        pi += (4.0 / div) * mult;
        div += 2.0;
        mult *= -1.0;
    }
    pi
}
