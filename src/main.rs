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

//TODO: redefine as recursive macro
fn fact(x: u32) -> u32 {
    let mut result: u32 = 1;
    for number in 2..(x+1) {
        result = result * number;
    }
    result
}

fn main() {
    let x: u32 = 4;
    println!("{}! = {}", x, fact(x));
}
