fn main() {
	let P:f64 = 520000000.0;
    let R:f64 = 10.0;
    let n:f64 = 5.0;

    // compound interest
    let A = P * ( 1.0 + (R / 100.0)) * n;
    println!("Amount is {}", A);
    let ci = A - P;
    println!("Compound interest is {}", ci);

}