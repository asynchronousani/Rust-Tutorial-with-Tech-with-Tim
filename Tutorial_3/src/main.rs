//Rust Tutorial #3 - Variables, Constants and Shadowing

fn main() {
    let x = 4;
    println!("x is: {}", x);
    
    {
        let x = x - 2;
        println!("x is: {}", x);
    }
    
    let x = x + 1;
    println!("x is: {}", x);
    
    let x = "hello";
    println!("x is: {}", x);
    
    const SECONDS_TO_MINUTES: u32 = 60;
    println!("{}", SECONDS_TO_MINUTES);
}
