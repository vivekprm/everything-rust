fn sum(numbers: &[i32]) -> i32 {
    let mut sum = 0;
    for num in numbers {
        sum += num;
    }
    sum
}
fn main() {
    let numbers = [1, 2, 3, 4, 5];
    let result = sum(&numbers);
    println!("Sum is: {}", result);
}
