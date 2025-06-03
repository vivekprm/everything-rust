use std::io;

fn main() {
    println!("Enter the temperature in fahrenheit:");
    let mut temp = String::new();

    io::stdin()
        .read_line(&mut temp)
        .expect("Invalid temperature");

    let temp: f32 = temp.trim().parse().expect("invalid temperature value");
    let tc = convert(temp);
    println!("Temperature in celsius is: {}", tc);

    println!("Enter the number to calculate fibonacci for:");

    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Enter the valid number");

    let n: i32 = n.trim().parse().expect("Invalid number");
    let fib = fib(n);
    println!("{}th Fibonacci number is: {}", n, fib);
}

fn convert(temp: f32) -> f32 {
    (5.0 * (temp - 32.0)) / 9.0
}

fn fib(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }
    fib(n - 1) + fib(n - 2)
}
