use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fmt::{Display, Formatter};
use std::hash::Hash;



pub struct Graph{
    pub graph: HashMap<Node,Vec<Edge>>,
}

impl Graph{
    pub fn new() -> Self{
        Self{
            graph: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node:Node){
        self.graph.entry(node).or_insert(Vec::new());
    }

    pub fn add_edge(&mut self, node: Node,edge: Edge){
        let x = self.graph.entry(node).or_insert(Vec::new());
        if !x.contains(&edge){
            x.push(edge);
        }
    }

    pub fn print_graph(&self){
        for i in &self.graph{
            print!("{} : ",i.0);
            for j in i.1{
                print!("-> ");
                print!("{},{},{}",j.node,j.location,j.train_no);
            }
            println!()
        }
    }

    pub fn search_graph(&mut self, start: Node, goal: Node) -> Option<Vec<Edge>>{
        let mut frontier = BinaryHeap::new();
        let mut distance_tracker = HashMap::new();
        for i in self.graph.keys(){
            distance_tracker.insert(i.clone(),usize::MAX);
        }
        let x = distance_tracker.entry(start.clone()).or_insert(0);
        *x =0;
        let mut pair = Pair::new(start.clone());
        let edge = Edge::new(start.clone(),1,String::new(),0);
        pair.store.push(edge);
        frontier.push(pair);
        while let Some(p) =frontier.pop(){
            if p.node == goal{
                return Some(p.store);
            }
            if p.sum_of_cost()-1 > distance_tracker.get(&p.node).copied().unwrap(){
                continue;
            }
            for edge in self.graph.get_mut(&p.node).unwrap(){
                let mut next = Pair::new(edge.node.clone());
                next.store.extend(p.store.clone());
                next.store.push(edge.clone());
                let x = next.sum_of_cost() -1;
                if x < distance_tracker.get(&edge.node).copied().unwrap(){
                    *distance_tracker.entry(edge.node.clone()).or_insert(0) = x;
                    frontier.push(next);
                }
            }
        }


        None
    }


}

#[derive(Debug,Hash,PartialOrd, PartialEq,Ord, Eq,Clone)]
pub struct Edge {
    node: Node,
    cost: usize,
    pub train_no : String,
    pub location: u32,
}

impl Edge{
    pub fn new(node: Node,cost: usize,train_no:String,location:u32) -> Self{
        Self{
            node,
            cost,
            train_no,
            location,
        }
    }

}

#[derive(Hash,PartialOrd, PartialEq,Ord,Eq,Debug,Clone)]
pub struct Node{
    station: String,
}

impl Node {
    pub fn new(station: String) -> Self{
        Self{
            station,
        }
    }
}

impl Display for Node{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.station)
    }
}

#[derive(Hash,Eq,PartialEq)]
struct Pair{
    node: Node,
    store: Vec<Edge>,
}

impl Pair {
    pub fn new(node: Node) -> Self{
        Self{
            node,
            store: Vec::new(),

        }
    }

    pub fn sum_of_cost(&self) -> usize{
        self.store.iter().map(|s| s.cost).sum()
    }
}

impl PartialOrd for Pair{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.store.len().cmp(&self.store.len()))
    }
}

impl Ord for Pair{
    fn cmp(&self, other: &Self) -> Ordering {
        other.store.len().cmp(&self.store.len())
    }
}