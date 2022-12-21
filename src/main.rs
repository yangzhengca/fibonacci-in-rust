use std::io;

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n-1) + fibonacci(n-2),
    }
}

fn main() {
    println!("Hello, Fibonacci!");
    println!("Please type a positive integer:");

    let mut user_input = String::new();
    
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");
    
    let user_input: u32 = user_input.trim().parse().expect("Please type a integer!");

    println!("Your input is: {user_input}");
    
    println!("The fabonacci number is {}", fibonacci(user_input));   
        
}
       
