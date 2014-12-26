extern crate integrate;

use integrate::cquad;
use std::num::FloatMath;
use std::f64::consts;

fn main() {
    // Integrate sin(x)/2 from 0 to pi
    let (result, err, neval) = cquad(|x| 0.5f64 * FloatMath::sin(x), 0f64, consts::PI);
    println!("result: {:.10e}, err {:.10e}, neval {}", result, err, neval);
}

