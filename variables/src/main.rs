fn main() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("Inner Scope: {}", x);
    }
    println!("Outer Scope: {}", x);
    another_function(x);
}

fn another_function(y: i32) {
    println!("Hello from another function!");
    println!("The parameter is {y}")
}