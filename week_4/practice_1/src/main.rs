fn main() {
    let name = "Aisha Lawal";
    let uni:&str = "Pan-Atlantic University";
    let addr: &str = "Km 52 Lekki-Epe Expressway, Ibeju-Lekki, lagos";
    println!("Name:{}",name );
    println!("University: {}.\nAddress: {}", uni,addr);

    let department:& str = "Computer Science";
    let school:&'static str = "School of Science and Technology";

    println!("Department: {}, \nSchool: {}",department,school );
}