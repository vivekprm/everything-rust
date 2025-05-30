#  Graph
## Centrality or Closeness Centrality
Measure that indicates the average distance between a node and all the other nodes in the  Network.

```rust
use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::Direction;
use std::fmt;

#[derive(Debug)]
struct Fighter {
    name: String,
}

impl Fighter {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl fmt::Display for Fighter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

fn add_edge(graph: &mut UnGraph<&Fighter, f32>, nodes: &[NodeIndex], a: usize, b: usize) {
    graph.add_edge(nodes[a], nodes[b], 1.0);
}

fn main() {
    let mut graph = UnGraph::new_undirected();
    let fighters = [
        Fighter::new("Dustin Poirier"),
        Fighter::new("Khabib Nurmagomedov"),
        Fighter::new("Jose Aldo"),
        Fighter::new("Conor McGregor"),
        Fighter::new("Nat Diaz"),
    ];

    let figher_nodes: Vec<NodeIndex> = fighters
        .iter()
        .map(|fighter| graph.add_node(fighter))
        .collect();

    add_edge(&mut graph, &figher_nodes, 0, 1);
    add_edge(&mut graph, &figher_nodes, 1, 3);
    add_edge(&mut graph, &figher_nodes, 3, 0);
    add_edge(&mut graph, &figher_nodes, 3, 2);
    add_edge(&mut graph, &figher_nodes, 3, 4);
    add_edge(&mut graph, &figher_nodes, 0, 4);
    add_edge(&mut graph, &figher_nodes, 2, 4);

    for (i, &node) in figher_nodes.iter().enumerate() {
        let name = &fighters[i].name;
        let degree = graph.edges_directed(node, Direction::Outgoing).count() as f32;
        let closeness = 1.0 / degree;
        println!("The closeness centrality of {} is {:.2}", name, closeness);

        // Explaination
        match name.as_str() {
            "Conor McGregor" => {
                println!(
                    "{} has the lowest centrality, becuase he has fought with all other fighters",
                    name
                );
            }
            "Dustin Poirier" | "Nate Diaz" => {
                println!(
                    "{} has centrality of {:.2}, implying they had less fights compared to Conor McGregor",
                    name, closeness
                );
            }
            "Khabib Nurmagomedov" | "Jose Aldo" => {
                println!(
                    "{} has the highest centrality of {:.2} as they have fought with the least number of fighters",
                    name, closeness
                );
            }
            _ => {}
        }
        println!("-----------------------------------");
    }
}
```

# HashSet
## Storing unique fruits using HashSet in Rust
```rust
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashSet;

fn generate_fruit() -> &'static str {
    let fruits = [
        "Apple",
        "Banana",
        "Cherry",
        "Date",
        "Elderberry",
        "Fig",
        "Grape",
        "Honeydew",
    ];
    let mut rng = thread_rng();
    fruits.choose(&mut rng).unwrap()
}

fn main() {
    let mut fruit_set = HashSet::new();
    println!("Generating 100 random fruits...");
    for _ in 0..100 {
        fruit_set.insert(generate_fruit());
    }
    println!("Number of unique fruits generated: {}", fruit_set.len());
}
```

# BTreeSet
## Maintaining Sorted & Unique Fruits Using BTreeSet 
```rust
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::BTreeSet;

fn main() {
    let fruits = vec![
        "Apple",
        "Banana",
        "Cherry",
        "Date",
        "Elderberry",
        "Fig",
        "Grape",
        "Honeydew",
    ];
    let amounts = [1, 3, 5, 7, 9];
    let mut rng = thread_rng();
    for amount in amounts.iter() {
        let mut fruit_set = BTreeSet::new();
        let mut shuffled_fruits = fruits.clone();
        shuffled_fruits.shuffle(&mut rng);

        for fruit in shuffled_fruits {
            fruit_set.insert(fruit);
            if fruit_set.len() >= *amount {
                break;
            }
        }
        println!("{}: {:?}", amount, fruit_set);
    }
}
```

# Binary Heap
## Creating a fig-priority fruit salad using BinaryHeap
```rust
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
```