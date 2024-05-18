use std::path::PathBuf;
use djkstra_shortest_path::{create_graph, parse_file, Problem, process_pair};
use djkstra_shortest_path::graph::{Node, Pair};

fn main() {
    //parse_file("example-problems.csv").expect("Couldnot work");
  let mut  path = PathBuf::new();
    path.push("mini-schedule.csv");
   let mut x = create_graph(&path,"stops").unwrap();
    //x.print_graph();
   let mut y = x.search_graph(Node::new("MDS".to_string()), Node::new("DURG".to_string()),Some("price")).unwrap();
    println!("{}",y.sum_of_cost_another());
    println!("{:?}",y.store);
    let z = process_pair(&mut y.store);
    println!("{}",z);

}