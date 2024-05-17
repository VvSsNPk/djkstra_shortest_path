use std::path::PathBuf;
use djkstra_shortest_path::{create_graph, parse_file, Problem, process_pair};
use djkstra_shortest_path::graph::{Node, Pair};

fn main() {
    parse_file("example-problems.csv").expect("Couldnot work");
/*    let mut  path = PathBuf::new();
    path.push("mini-schedule.csv");
   let mut x = create_graph(&path,"stops").unwrap();
    let mut y = x.search_graph(Node::new("BUI".to_string()), Node::new("CBN".to_string())).unwrap();
    println!("{}",y.sum_of_cost());
    println!("{:?}",y.store);
    let z = process_pair(&mut y.store);
    println!("{}",z);*/
    //println!("{}",z);
/*    let mut grouped_items = Vec::new();
    let mut current_group = Vec::new();
    let mut current_value = ans.get(0).unwrap().train_no.clone();*/

/*    for item in ans{
        if item.train_no == current_value{
            current_group.push(item.location);
        }else{
            grouped_items.push((current_value,current_group));
            current_group = vec![item.location];
            current_value = item.train_no.clone();
        }
    }*/
/*    for i in grouped_items{
        print!("{} : {} -> {};",i.0.strip_prefix("'").unwrap().strip_suffix("'").unwrap(),i.1.first().unwrap()-1,i.1.last().unwrap());
    }*/
   /* let mut reader = csv::Reader::from_path("example-problems.csv").unwrap();
    for i in reader.deserialize(){
        let r : Problem = i.unwrap();
        println!("{:?}",r);
    }*/

}

/*fn change_pair(pair: &mut Pair){
    if pair.store.len() > 2{
        let  y = pair.store.get_mut(1).unwrap().clone();
        let x = pair.store.get_mut(0).unwrap();
        x.train_no = y.train_no.clone().to_string();
        x.location = y.location-1;
    }
}*/
