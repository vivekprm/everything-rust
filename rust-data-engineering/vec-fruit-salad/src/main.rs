use rand::seq::SliceRandom; // rand is random number generation library
use rand::thread_rng;
/**
 * This program creates a fruit salad by scrambling a list of fruits.
 * A vector is a growable array. It can grow or  shrink in  size and is  one of the  most
 * useful datastructures in Rust. The `vec!` macro is used to create a vector. A vector is
 * represented using the Vec<T> type.
 */
use std::vec;

fn main() {
    let mut fruit = vec!["Orange", "Fig", "Pomegranate", "Cherry", "Apple", "Grape"];
    // Shuffle the fruits
    let mut rng = thread_rng();
    fruit.shuffle(&mut rng);

    // Printout the fruit salad
    println!("Fruit Salad:");
    for (i, item) in fruit.iter().enumerate() {
        if i != fruit.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}", item);
        }
    }
}
