// Importing  the  fill funnction from the textwrap crate to wrap text at 78 characters per
// line.
use textwrap::fill;

// The pagerank struct holds the damping factor annd the number of iterations to run the algorithm.
struct PageRank {
    damping: f64,
    iterations: usize,
}

impl PageRank {
    // The new function creates a new instance of the PageRank struct.
    fn new(damping: f64, iterations: usize) -> Self {
        Self {
            damping,
            iterations,
        }
    }

    // The rank function calculates and returns the page rank of each node in the graph.
    fn rank(&self, graph: &Vec<Vec<usize>>) -> Vec<f64> {
        // The number of nodes in the graph.
        let n = graph.len();

        // The initial page rank of each node.
        let mut ranks = vec![1.0 / (n as f64); n];

        // Iterates the specified number of times.
        for _ in 0..self.iterations {
            // A new vector to hold the updated PageRank values.
            let mut new_ranks = vec![0.0; n];

            // Iterate over each node and it's edges in the graph.
            for (node, edges) in graph.iter().enumerate() {
                // The amount of PageRank value this node contributes to its liked nodes.
                let contribution = ranks[node] / (edges.len() as f64);

                // Distribute the PageRank value to the linked nodes.
                for &edge in edges {
                    new_ranks[edge] += contribution;
                }
            }

            // Updates the PageRank values using the damping factor.
            for rank in &mut new_ranks {
                *rank = *rank * self.damping + (1.0 - self.damping) / (n as f64);
            }

            // Replaces the old page rank values with the new ones.
            ranks = new_ranks;
        }
        // Returns the final PageRank values.
        ranks
    }
}

fn main() {
    // The graph represents links between sports websites. Each index represents a website,
    // and the values in the vectors are inndexes of the websites they link to.
    let graph = vec![
        vec![1, 2], // ESPN links to NFL, NBA
        vec![0],    // NFL links to ESPN
        vec![0, 3], // NBA links to ESPN, UFC
        vec![0],    // UFC links to ESPN
        vec![0, 1], //  MLB links to ESPN, NFL
    ];

    // The names corresponding to the indexes of the websites.
    let names = vec!["ESPN", "NFL", "NBA", "UFC", "MLB"];

    // Initializes the PageRank struct.
    let pagerank = PageRank::new(0.85, 100);

    // Calculates the PageRank values.
    let ranks = pagerank.rank(&graph);

    // Prints the PageRank values.
    for (i, rank) in ranks.iter().enumerate() {
        println!("The pagerank of {} is {}", names[i], rank);
    }

    // Explaination of how PageRank works.
    let explaination = "PageRank is a link analysis algorithm used by Google that uses the hyperlink structure of the web to determine a quality ranking for each web page. It works by counting the number and quality of links to a page to determine a rough estimate of how important the website is. The underlying assumption is that more important websites are likely to receive more links from other websites. The algorithm is iterative and converges to a stable value after a number of iterations. The damping factor is used to model the probability that a user will continue clicking on links on a page.";

    // Prints the explanation wrapped at 78 characters per line.
    println!("{}", fill(explaination, 78));
}
