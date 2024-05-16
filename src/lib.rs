use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;
use chrono::{NaiveTime, TimeDelta};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use crate::graph::{Edge, Graph, Node, Pair};

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


#[serde_as]
#[derive(Serialize,Deserialize,Debug)]
pub struct Problem{
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
#[serde_as]
#[derive(Serialize,Deserialize,Debug)]
struct Solution{
    #[serde(rename="ProblemNo")]
    problem_no : usize,
    #[serde(rename="Connection")]
    connection : String,
    #[serde(rename="Cost")]
    cost : usize,
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
            let (node,_number,code,_islno,_,current_departure) = i[j].clone();
            let (node2, number2, code2, islno2,next_arrivat,_) = i[j+1].clone();
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
            }else{
                continue;
            }
            let x = cost.num_seconds();
            graph.add_node(Node::new(code.clone()));
            graph.add_node(Node::new(code2.clone()));
            let edge = Edge::new(Node::new(code2), x as usize, number2, islno2);
            graph.add_edge(Node::new(code),edge);
        }
    }
    Ok(graph)
}

pub fn parse_file(str: &str) -> Result<(),Box<dyn Error>>{
    let mut problems_file = PathBuf::new();
    problems_file.push(str);
    let mut reader = csv::Reader::from_path(problems_file)?;
    let mut solutions_file = PathBuf::new();
    let mut writer = csv::Writer::from_path(solutions_file)?;
    for prob in reader.deserialize(){
        let problem : Problem = prob?;
        let mut schedule_file = PathBuf::new();
        schedule_file.push(problem.file.trim());
        let mut graph = create_graph(&schedule_file,problem.cost.as_str())?;
        let mut pair = graph.search_graph(Node::new(problem.from),Node::new(problem.to)).unwrap();



    }


    Ok(())
}


fn process_pair( pair: &mut Vec<Edge>) -> String{
    pair.remove(0);
    let mut grouped_items = Vec::new();
    let mut current_group = Vec::new();
    let mut current_value = pair.get(0).unwrap().train_no.clone();

    for item in pair{
        if item.train_no == current_value{
            current_group.push(item.location);
        }else{
            grouped_items.push((current_value,current_group));
            current_group = vec![item.location];
            current_value = item.train_no.clone();
        }
    }


    String::new()
}



