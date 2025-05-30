/**
 * This code starts with an initial VecDeque.
 * converts it to a Vec for shuffling, and then converts it back to a VecDeque.
 * After that it pushes "Pomegranate" to the front of the Deque, and "Fig" and "Cherry" to
 * the back of the Deque. Finally, it prints out the final fruit salad.
 */
/**
 * VecDeque is a double-ended queue implemented with a growable ring buffer. Which means
 * that you can push and pop elements from both ends.
 */
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::VecDeque;

fn main() {
    let mut fruit = VecDeque::new();
    fruit.push_back("Arbutus");
    fruit.push_back("Loquat");
    fruit.push_back("Strawberry Tree Berry");

    // Scramble(shuffle) the fruit
    let mut rng = thread_rng();
    let mut fruit: Vec<&str> = fruit.into_iter().collect();
    fruit.shuffle(&mut rng);

    // Convert it back to a VecDeque
    let mut fruit: VecDeque<&str> = fruit.into_iter().collect();

    // Add  fruits  to the both  ends of the queue after  shuffling
    fruit.push_front("Pomegranate");
    fruit.push_back("Fig");
    fruit.push_back("Cherry");

    // Print out the final fruit salad
    for (i, item) in fruit.iter().enumerate() {
        if i != fruit.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}", item);
        }
    }
}
