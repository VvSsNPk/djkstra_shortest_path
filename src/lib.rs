use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;
use serde::Deserialize;
use serde_with::serde_as;
use crate::graph::{Edge, Graph, Node};

pub mod graph;
#[serde_as]
#[derive(Debug,Deserialize,Hash,PartialOrd, PartialEq,Ord, Eq,Clone)]
struct Record{
    #[serde(rename="Train No.")]
    train_number : String,
    #[serde(rename="train Name")]
    train_name : String,
    #[serde(rename="islno")]
    islno : u32,
    #[serde(rename="station Code")]
    station_code: String,
    #[serde(rename="Station Name")]
    station_name: String,
    #[serde(rename="Arrival time")]
    arrival_time: String,
    #[serde(rename="Departure time")]
    departure_time: String,
    #[serde(rename="Distance")]
    distance: u32,
    #[serde(rename="Source Station Code")]
    source_station_code: String,
    #[serde(rename="source Station Name")]
    source_station_name: String,
    #[serde(rename="Destination station Code")]
    destination_station_code: String,
    #[serde(rename="Destination Station Name")]
    destination_station_name: String,
}

pub fn create_graph(path: &PathBuf) -> Result<Graph,Box<dyn Error>>{
    let mut file = csv::Reader::from_path(path)?;
    let mut graph =  Graph::new();
    let mut map = HashMap::new();
    for result in file.deserialize(){
        let mut record:Record = result?;
        record.destination_station_code= record.destination_station_code.trim().parse().unwrap();
        record.source_station_name=record.source_station_name.trim().parse().unwrap();
        record.station_name=record.station_name.trim().parse().unwrap();
        record.train_name=record.train_name.trim().parse().unwrap();
        record.station_code=record.station_code.trim().parse().unwrap();
        record.source_station_code=record.destination_station_code.trim().parse().unwrap();
        let mut x = map.entry(record.train_name.clone()).or_insert(Vec::new());
        x.push((record.station_name,record.train_number,record.station_code,record.islno))
    }
    for i in map.values(){
        for j in 0..(i.len()-1){
            let (node,number,code,islno) = i[j].clone();
            let (node2, number2, code2, islno2) = i[j+1].clone();
            graph.add_node(Node::new(node.clone()));
            graph.add_node(Node::new(node2.clone()));
            let edge = Edge::new(Node::new(node2),1,number2,islno2);
            graph.add_edge(Node::new(node),edge);
        }
    }
    Ok(graph)
}