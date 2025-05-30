Sequence in Rust is like Python List,  and  Map is  like  Python dictionary. But one of the key differences is that they're immutable  by default.

```rust
fn main() {
    let fruits = vec!["Apple", "Banana", "Cherry"];
    for fruit in &fruits {
        println!("{}", fruit);
    }

    let map = std::collections::HashMap::new();
    map.insert("key", "value"); // this will cause compile time error
}
```

# Print Rust data structure
```rust
fn main() {
    println!("Common Rust Collections:");

    // Sequences
    println!("\n\tSequences:");
    println!("\t\tVec: https://doc.rust-lang.org/std/vec/struct.Vec.html");
    println!(
        "\t\tVecDeque: https://doc.rust-lang.org/std/collections/vec_deque/struct.VecDeque.html"
    );
    println!("\t\tLinkedList: https://doc.rust-lang.org/std/collections/linked_list/struct.LinkedList.html");

    // Maps
    println!("\n\tMaps:");
    println!("\t\tHashMap: https://doc.rust-lang.org/std/collections/struct.HashMap.html");
    println!("\t\tBTreeMap: https://doc.rust-lang.org/std/collections/struct.BTreeMap.html");

    // Sets
    println!("\n\tSets:");
    println!("\t\tHashSet: https://doc.rust-lang.org/std/collections/struct.HashSet.html");
    println!("\t\tBTreeSet: https://doc.rust-lang.org/std/collections/struct.BTreeSet.html");

    // Misc
    println!("\n\tMisc:");
    println!("\t\tBinaryHeap: https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html");
}
```

## Vector
```rust
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
```

## Vec Dequeue
Double ended queue similar to python's  Dequeue. Cool thing  about it is, it has O(1) time complexity for operations on  both ends.

```rust
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
    let mut fruit = fruit.into_iter().collect::<Vec<&str>>();
    fruit.shuffle(&mut rng);

    // Convert it back to a VecDeque
    let mut fruit = fruit.into_iter().collect::<VecDeque<&str>>();

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
```

## LinkedList
```rust
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
```

# Creating command line tool
add below ```lib.rs``` file
```rust
use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn create_fruit_salad(num_fruits: usize) -> Vec<String> {
    let mut fruits = vec![
        "Arbutus".to_string(),
        "Loquat".to_string(),
        "Strawberry Tree Berry".to_string(),
        "Pomegranate".to_string(),
        "Fig".to_string(),
        "Cherry".to_string(),
        "Orange".to_string(),
        "Pear".to_string(),
        "Peache".to_string(),
        "Apple".to_string(),
    ];

    let mut rng = thread_rng();
    fruits.shuffle(&mut rng);

    fruits.into_iter().take(num_fruits).collect()
}
```

Add below ```main.rs```
```rust
use clap::Parser;
use cli_salad::create_fruit_salad;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Vivek Mishra",
    about = "Number of fruits to include in the salad"
)]
struct Opts {
    #[clap(short, long, default_value = "3")]
    number: usize,
}

fn main() {
    let opts: Opts = Opts::parse();
    let num_fruits = opts.number;
    let salad = create_fruit_salad(num_fruits);
    println!(
        "Created Fruit Salad with {} fruits: {:?}",
        num_fruits, salad
    );
}
```

To run it, use below command:
```sh
cargo run --  --number 5
```

To get help
```sh
cargo run --  --help
```

# Hashmap Frequency Counter
Hashmap is  very similar to python dictionary. It had  O(1) complexity for insertion, retrieval, deletion.

```rust
/**
 * This example code counts the frequency of each number in the vector.
 */
use std::collections::HashMap;

fn logic(numbers: Vec<i32>) -> Vec<(i32, u32)> {
    let mut frequencies = HashMap::new();
    for num in numbers {
        let frequency = frequencies.entry(num).or_insert(0);
        *frequency += 1;
    }
    let mut result = Vec::new();

    for (num, frequency) in frequencies {
        result.push((num, frequency));
    }
    result
}
fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 3];
    let result = logic(numbers);
    println!(
        "The frequency of each number in the vector is: {:?}",
        result
    );
}
```

# Hashmap Language Comparison
```rust
use std::collections::HashMap;

fn init_languages() -> HashMap<String, i32> {
    let mut languages = HashMap::new();
    languages.insert("JavaScript".to_string(), 1995);
    languages.insert("HTML/CSS".to_string(), 1993);
    languages.insert("Python".to_string(), 1991);
    languages.insert("SQL".to_string(), 1974);
    languages.insert("TypeScript".to_string(), 2012);
    languages.insert("Bash/Shell".to_string(), 1989);
    languages.insert("Java".to_string(), 1995);
    languages.insert("C#".to_string(), 2000);
    languages.insert("C++".to_string(), 1983);
    languages.insert("C".to_string(), 1972);
    languages.insert("PHP".to_string(), 1995);
    languages.insert("PowerShell".to_string(), 2006);
    languages.insert("Go".to_string(), 2007);
    languages.insert("Rust".to_string(), 2010);
    languages
}

fn calculate_weight(years_active: &mut HashMap<String, i32>) -> HashMap<String, i32> {
    // Substract  the creation year from 2024
    for year in years_active.values_mut() {
        *year = 2024 - *year;
    }

    let min_year = *years_active.values().min().unwrap_or(&0);
    let max_year = *years_active.values().max().unwrap_or(&0);

    let mut weights = HashMap::new();
    for (language, year) in years_active.iter() {
        let normalized_year = (year - min_year) as f64 / (max_year - min_year) as f64;
        let weight = (normalized_year * 99.0) as i32 + 1; // Weights between 1 and 100
        weights.insert(language.to_string(), weight);
    }
    weights
}

fn main() {
    let mut languages = init_languages();
    let weights = calculate_weight(&mut languages);
    println!("Language weighing from 1-100 by age (1 is newest and 100 is oldest):");
    for (language, weight) in &weights {
        println!("{}: {}", language, weight);
    }
}
```