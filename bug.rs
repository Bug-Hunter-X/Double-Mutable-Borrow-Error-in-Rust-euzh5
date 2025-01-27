fn main() {
    let mut x = 5;
    let y = &mut x; 
    let z = &mut x; // This is the error, double mutable borrow
    *y = 10;
    *z = 12;
    println!("x = {}", x);
}