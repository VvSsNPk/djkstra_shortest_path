use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use djkstra_shortest_path::{create_graph};
use djkstra_shortest_path::graph::{Node, Pair};

#[serde_as]
#[derive(Serialize,Deserialize,Debug)]
struct Problem{
    #[serde(rename="ProblemNo")]
    problem : usize,
    #[serde(rename="FromStation")]
    from : String,
    #[serde(rename="ToStation")]
    to: String,
    #[serde(rename="Schedule")]
    file : String,
    #[serde(rename="CostFunction")]
    cost : String,

}
fn main() {
    let mut  path = PathBuf::new();
    path.push("schedule.csv");
   let mut x = create_graph(&path,"traveltime").unwrap();
    let y = x.search_graph(Node::new("KAZHAKUTTAM".to_string()),Node::new("KATNI SOUTH".to_string())).unwrap();
    println!("{}",y.sum_of_cost());
    println!("{:?}",y.store);
    /*let mut reader = csv::Reader::from_path(path).unwrap();
    for i in reader.deserialize(){
        let r : Problem = i.unwrap();
        println!("{:?}",r);
    }*/

}

fn change_pair(pair: &mut Pair){
    if pair.store.len() > 2{
        let  y = pair.store.get_mut(1).unwrap().clone();
        let x = pair.store.get_mut(0).unwrap();
        x.train_no = y.train_no.clone().to_string();
        x.location = y.location-1;
    }
}
