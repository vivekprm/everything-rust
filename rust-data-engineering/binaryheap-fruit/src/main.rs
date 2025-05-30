use rand::seq::SliceRandom;
use rand::thread_rng;
use std::cmp::Ord;
use std::collections::BinaryHeap;

#[derive(Eq, PartialEq)]
enum Fruit {
    Fig,
    Other(String),
}

// We define Figs as the highest priority by implementing the Ord trait
impl Ord for Fruit {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Fruit::Fig, Fruit::Fig) => std::cmp::Ordering::Equal,
            (Fruit::Fig, _) => std::cmp::Ordering::Greater,
            (_, Fruit::Fig) => std::cmp::Ordering::Less,
            (Fruit::Other(_), Fruit::Other(_)) => std::cmp::Ordering::Equal,
        }
    }
}

impl PartialOrd for Fruit {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn generate_fruit_salad() -> BinaryHeap<Fruit> {
    let mut rng = thread_rng();
    let fruits = vec![
        "Apple", "Orange", "Pear", "Peach", "Banana", "Fig", "Fig", "Fig", "Fig",
    ];
    let mut fruit_salad = BinaryHeap::new();
    let mut fig_count = 0;
    while fig_count < 2 {
        let fruit = fruits.choose(&mut rng).unwrap();
        if *fruit == "Fig" {
            fig_count += 1;
            fruit_salad.push(Fruit::Fig);
        } else {
            fruit_salad.push(Fruit::Other(fruit.to_string()));
        }
    }
    fruit_salad
}

fn main() {
    println!("Random Fruit Salad With Two Servings of Figs:");
    let fruit_salad = generate_fruit_salad();
    for fruit in fruit_salad {
        match fruit {
            Fruit::Fig => println!("Fig"),
            Fruit::Other(fruit) => println!("{}", fruit),
        }
    }
}
