// Rust program to count numbers

use std::io;

fn main(){

   println! ("Enter lower bound");
   let mut input1 = String::new();
   io::stdin().read_line(&mut input1).expect("Failed to read input");
   Let lower_bound: i32 = input1trim().parse().expect("Failed to input");

   println! ("Enter upper bound");
   let mut input2 = String: :new();
   io::stdin().read_line(&mut input2).expect("Failed to read input");
   let upper_ bound: 132 - input2.trim().parse().expect("Failed to input"); 

   for x in lower_ bound..upper_bound{ // upper_bound is not inclusive
         
     println! ("Count Level is {}",×);
   }
}
