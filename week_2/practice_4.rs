fn main() {
	let P:f64 = 210000.0;
    let R:f64 = 5.0;
    let n:f64 = 3.0;

    // compound interest
    let A = P * ( 1.0 - (R / 100.0)) * n;
    println!("Amount is {}", A);
}