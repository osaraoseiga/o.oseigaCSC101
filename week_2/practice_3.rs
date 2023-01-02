fn main() {
	let Toshiba:f64 = ( 2.0 * 450000.0 );
    let Mac:f64 = ( 1.0 * 1500000.0 );
    let Hp:f64 = ( 3.0 * 750000.0 );
    let Dell:f64 = (3.0 * 2850000.0 );
    let Acer:f64 = ( 1.0 * 250000.0 );

    // Sum and Average
    let S = ( Toshiba + Mac + Hp + Dell + Acer );
    println!("Sum is {}", S);
    let A = ( S / 5.0 );
    println!("Average is{}", A);
}  


