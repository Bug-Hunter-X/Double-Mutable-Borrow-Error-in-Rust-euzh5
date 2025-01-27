fn main() {
    let mut x = 5;
    { //Creating a scope to limit lifetime of y
        let y = &mut x;
        *y = 10;
        println!("x = {}", x);
    }
    let z = &mut x;
    *z = 12;
    println!("x = {}", x);
}