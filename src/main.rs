/*
 * Christopher Leap
 * 2018-08-30
 *
 * Exponent Approximation : 
 *      Approximates e^x using the Maclaurin Series expansion:
 * 
 *       ∞
 * e^x = ∑ [ x^i / i! ]
 *      i=0
 */

use std::io;

// fact : takes in an unsigned int x and returns x!
fn fact(x: u32) -> u32 {
    let mut result: u32 = 1;
    for number in 2..(x+1) {
        result = result * number;
    }
    result
}

// pow_basic : returns base^power
fn pow_basic(base: f64, power: u32) -> f64 {
    let mut result: f64 = 1.0;
    for number in 0..power {
        result = result * base;
    }
    result
}

// compute : computes the Maclaurin series for e^x for the given x, running
//          n terms of the Maclaurin series expansion
fn compute(x: f64, n: u32) -> f64 {
    let mut result: f64 = 0.0;
    for term in 0..(n+1) {
        result = result + (pow_basic(x, term)) / (fact(term) as f64);
    }
    result
}

// main : this is where all the magic happens
fn main() {
    println!("Hello! Thrilled to receive an approximate value for e^x, I see!");
    println!("What value would you like for x?");

    let mut x = String::new();
    io::stdin().read_line(&mut x)
        .expect("Failed to read line");
    let x: f64 = x.trim().parse()
        .expect("Please type a number!");

    println!("x is {} - excellent!", x);
    println!("How many degrees would you like for the Maclauren Series?");

    let mut n = String::new();
    io::stdin().read_line(&mut n)
        .expect("Failed to read line");
    let n: u32 = n.trim().parse()
        .expect("Please type a number!");

    println!("Computing the {} degree Maclauren Series expansion for {}", n, x);
    println!("e^{} is approximately equal to {}", x, compute(x, n));
}
