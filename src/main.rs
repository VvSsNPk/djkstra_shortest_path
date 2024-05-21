use std::path::PathBuf;
use djkstra_shortest_path::{create_graph_one, create_graph_two, parse_file, process_pair, process_pair_two};
use djkstra_shortest_path::graph::{Node, Pair};

fn main() {
    //parse_file("example-problems.csv").expect("Couldnot work");
   let mut  path = PathBuf::new();
    path.push("mini-schedule.csv");
   let mut x = create_graph_two(&path).unwrap();
    //x.print_graph();
   let y = x.search_graph(Node::new("MDS".to_string()), Node::new("DURG".to_string()));
    let m = process_pair_two(&mut y.clone().unwrap().store);
    println!("{}",y.clone().unwrap().sum_of_cost());
    println!("{:?}",y.unwrap().store);
    println!("{}",m);
}