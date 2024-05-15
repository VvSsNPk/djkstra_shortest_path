use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;
use chrono::{NaiveTime, TimeDelta};
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

pub fn create_graph(path: &PathBuf,str: &str) -> Result<Graph,Box<dyn Error>>{
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
        record.arrival_time = record.arrival_time.strip_suffix("'").unwrap().strip_prefix("'").unwrap().to_string();
        record.departure_time = record.departure_time.strip_suffix("'").unwrap().strip_prefix("'").unwrap().to_string();
        let x = map.entry(record.train_name.clone()).or_insert(Vec::new());
        x.push((record.station_name,record.train_number,record.station_code,record.islno,record.arrival_time,record.departure_time));
    }
    for i in map.values(){
        for j in 0..(i.len()-1){
            let (node,_number,_code,_islno,_,current_departure) = i[j].clone();
            let (node2, number2, _code2, islno2,next_arrivat,_) = i[j+1].clone();
            let mut cost = TimeDelta::seconds(0) ;
            //println!("{}",current_departure);
            let time1 = NaiveTime::parse_from_str(current_departure.as_str(),"%H:%M:%S").unwrap();
            let time2 = NaiveTime::parse_from_str(next_arrivat.as_str(),"%H:%M:%S").unwrap();
            if str == "stops"{
                cost = TimeDelta::seconds(1);
            }else if str == "traveltime" {
                if time2 > time1 {
                    cost = time2 - time1
                } else {
                    cost = time2 - time1 + TimeDelta::try_hours(24).unwrap();
                }
            }
            let x = cost.num_seconds();
            graph.add_node(Node::new(node.clone()));
            graph.add_node(Node::new(node2.clone()));
            let edge = Edge::new(Node::new(node2), x as usize, number2, islno2);
            graph.add_edge(Node::new(node),edge);
        }
    }
    Ok(graph)
}