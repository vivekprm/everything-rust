# Detect Strongly Connected Component
```lib.rs```
```rust
pub const TWITTER_USERNAMES: [&str; 34] = [
    "blackmattersus",
    "bleepthepolice",
    "jenn_abrams",
    "leroylovesusa",
    "missourinewsus",
    "rightnpr",
    "ten_gop",
    "traceyhappymom",
    "trayneshacole",
    "traceyhappymom",
    "ten_gop",
    "leroylovesusa",
    "leroylovesusa",
    "traceyhappymom",
    "traceyhappymom",
    "traceyhappymom",
    "ten_gop",
    "ten_gop",
    "leroylovesusa",
    "leroylovesusa",
    "leroylovesusa",
    "ten_gop",
    "ten_gop",
    // fake community of journalists
    "journalist1",
    "journalist2",
    "journalist3",
    "journalist1",
    "journalist2",
    "journalist1",
    "journalist3",
    "journalist2",
    "journalist1",
    "journalist3",
    "journalist2",
];
```

```main.rs```
```rust
use community_detection::TWITTER_USERNAMES;
use petgraph::algo::kosaraju_scc;
use petgraph::prelude::*;
use std::collections::HashMap;

fn main() {
    // Create a new directed graph
    let mut graph = DiGraph::<&str, &str>::new();

    // Create a hashmap to store the node indices by username
    let mut nodes = HashMap::new();

    // Iterate over the data to populate the graph
    for window in TWITTER_USERNAMES.windows(2) {
        let user = window[0];
        let mention = window[1];

        // Add the nodes to the graph and to the hashmap
        let user_node = *nodes.entry(user).or_insert_with(|| graph.add_node(user));
        let mention_node = *nodes
            .entry(mention)
            .or_insert_with(|| graph.add_node(mention));

        // Add the edge to the graph
        graph.add_edge(user_node, mention_node, "retweets");
    }
    // Use the Kosarju's algorithm to find the strongly connected components
    let scc = kosaraju_scc(&graph);
    for component in scc {
        println!("{} nodes in community discovered", component.len());
        let usernames: Vec<&str> = component
            .iter()
            .map(|&node_index| graph[node_index])
            .collect();
        println!("{:?}", usernames);
    }
}
```