/**
 * As with the VecDeque example, this code starts by creating a LinkedList of fruits.
 * converts it to a Vec for shuffling, and then converts it back to a LinkedList.
 * After the shuffle, it adds "Pomegranate", "Fig" and "Cherry" to the end of the list.
 * Finally, it prints out the final fruit salad.
 *
 * This example  shows how to use a LinkedList,  but remember that LinkedList has a higher
 * memory overhead  and  worse cache locality  than Vec or VecDeque. So it's typically not
 * the best choice unless you have a specific need for the  properties of a linkedlist. In  
 * Rust, it's usually better to use Vec or VecDeque.
 *
 * LinkedList is a doubly linked list, which means each  element in the list has  a  
 * pointer  to the next  element and the previous element.
 * A great example of when to use a linkedlist is when you need to insert or remove elements
 * from the middle of the list frequently.
 */
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::LinkedList;

fn main() {
    let mut fruit: LinkedList<&str> = LinkedList::new();
    fruit.push_back("Arbutus");
    fruit.push_back("Loquat");
    fruit.push_back("Strawberry Tree Berry");

    // Scramble (shuffle) the fruit
    let mut rng = thread_rng();
    let mut fruit: Vec<_> = fruit.into_iter().collect();
    fruit.shuffle(&mut rng);

    // Convert it back to a LinkedList
    let mut fruit: LinkedList<_> = fruit.into_iter().collect();

    // Add fruits to the both ends of the list after shuffling
    fruit.push_back("Pomegranate");
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
