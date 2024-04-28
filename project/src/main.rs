mod io;
mod graph;
mod models;
mod clustering;
mod similarity;  
fn main() {
    let filepath = "C:/Users/Administrator/downloads/nba_2022_2023.csv";
    let players = io::read_players(filepath);  
    let graph = graph::build_graph(&players);  
    let avg_distance = graph::calculate_average_distance(&graph);
    println!("Average distance between nodes: {:.2}", avg_distance);
    graph::export_graph(&graph, "nba_graph.dot");
    println!("Graph has been exported to nba_graph.dot.");
    let representatives = clustering::find_representatives(&graph, 5);
    println!("Top 5 Representatives:");
    for node in representatives {
        if let Some(weight) = graph.node_weight(node) {
            println!("Player: {}", weight);
        }
    }
    let (most_similar, most_dissimilar) = similarity::find_extreme_similarity(&graph);
    println!("Most similar pair: ({:?}, {:?}) with Jaccard similarity {:.2}", most_similar.0, most_similar.1, most_similar.2);
    println!("Most dissimilar pair: ({:?}, {:?}) with Jaccard similarity {:.2}", most_dissimilar.0, most_dissimilar.1, most_dissimilar.2);
}

// Cited all the module in this project
// read the csv file ( you can change this path by yourself)
// Final, get the answer of the nodes graph, average distance, most similar or not similar, 
// and the clustering when K = 5  to determine which player is K 
