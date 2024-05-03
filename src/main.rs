mod graph;
use graph::Graph;
use std::fs::File;
use std::f64;
use std::io::BufRead;
use std::str::FromStr;
mod similarity;
use std::collections::{HashMap, HashSet, VecDeque};
#[derive(Debug,Clone)]
struct GameEntry {
    node_id: u64,
    game_id: u64,
    minute: f64, 
    description: String,
    home_club_goals: u64,
    away_club_goals: u64,
    total_goals: u64,
}
fn read_data(path: &str) -> HashMap<u64, Vec<GameEntry>>{
    let mut result = HashMap::new();
    let file = File::open(path).expect("Couldn't Open");
    let mut buf_reader = std::io::BufReader::new(file).lines();
    buf_reader.next();
    let mut prev_game_id = 100000000;
    let mut node_id = 0;
    for line in buf_reader {
        let line_str = line.expect("Error Reading");
        let events: Vec<String> = line_str.split(",").map(|s| s.to_string()).collect();
        let game_id = u64::from_str(&events[0]).unwrap();
        if prev_game_id != game_id { 
            node_id += 1;
        }
        prev_game_id=game_id;
        let game_entry = GameEntry {
            node_id,
            game_id: u64::from_str(&events[0]).unwrap(),
            minute: f64::from_str(&events[1]).unwrap(),
            description: events[5].clone(),
            home_club_goals: u64::from_str(&events[13]).unwrap(),
            away_club_goals: u64::from_str(&events[14]).unwrap(),
            total_goals: u64::from_str(&events[13]).unwrap() + u64::from_str(&events[14]).unwrap()
        };
        result.entry(game_entry.node_id).or_insert(Vec::new()).push(game_entry.clone());
        if node_id == 1000 {
            break
        }
    }
    result
}
fn make_edge_list(entry_map:&HashMap<u64,Vec<GameEntry>>) -> Vec<(f64, u64, u64)> {
    let mut edges: Vec<(f64,u64,u64)>=vec![];
    for (game_id1, game_entries1) in entry_map {
        for (game_id2, game_entries2) in entry_map {
            if game_id1 < game_id2 {
                let similarity_score = similarity::calculate_similarity(game_entries1.clone(), game_entries2.clone());
                if similarity_score>0.75 {
                    edges.push((similarity_score, *game_id1, *game_id2));
                }
            }
        }
    }
    edges
}
fn find_most_similar_game(edges: &Vec<(f64,u64,u64)>,entries: &HashMap<u64, Vec<GameEntry>>) -> (f64,Vec<GameEntry>,Vec<GameEntry>){
    let mut max_score = 0.0;
    let mut max_game1:&u64 = &0;
    let mut max_game2:&u64 = &0;
    for (score,game1,game2) in edges {
        if *score > max_score {
            max_score = *score;
            max_game1 = game1;
            max_game2 = game2;
        }
    }
    (max_score,entries[max_game1].clone(),entries[max_game2].clone())
} 
fn find_top_similar_games(edges: &Vec<(f64,u64,u64)>, entries: &HashMap<u64, Vec<GameEntry>>) -> Vec<(f64,Vec<GameEntry>,Vec<GameEntry>)> {
    let mut top_scores: Vec<(f64,u64,u64)> = Vec::new();
    for (score,game1,game2) in edges {
        top_scores.push((*score,*game1,*game2));
    }
    top_scores.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    let mut top_games: Vec<(f64,Vec<GameEntry>,Vec<GameEntry>)> = Vec::new();
    for (score,game1,game2) in top_scores.iter().take(10) {
        top_games.push((*score,entries[game1].clone(),entries[game2].clone()));
    }
    top_games
}
fn max_degree_centrality(graph:Graph) -> (u32,f64){
    let mut max_degree_centrality = 0.0;
    let mut max_degree_centrality_node = 0;
    let degree_centralities=graph.degree_centrality();
    for degree_centrality in degree_centralities.keys() { 
        if degree_centralities[degree_centrality] > max_degree_centrality {
            max_degree_centrality = degree_centralities[degree_centrality];
            max_degree_centrality_node = *degree_centrality;
        }
    }
    return (max_degree_centrality_node,max_degree_centrality)
}
fn main() {
   let entries=read_data("C:/Users/pje41/OneDrive/Desktop/soccer-data/processed_data/game_data.csv");
   let mut graph= Graph::new(entries.len() as usize);
   let edges: Vec<(f64,u64,u64)> = make_edge_list(&entries);
   for (weight,v, w) in &edges {
    graph.add_edge(*weight as f64,*v as usize, *w as usize);
   }
    let (max_score,game1,game2)=find_most_similar_game(&edges, &entries);
    println!("The highest similarity score was: {}",max_score);
    println!("The two games were: {:?} and {:?} \n Game 1 has the following events:",game1[0].game_id,game2[0].game_id);
    for i in 0..game1.len() {
        println!("{:?}",game1[i])
    }
    println!("Game 2 has the following events:");
    for i in 0..game2.len() {
        println!("{:?}",game2[i])
    }
    let top_games = find_top_similar_games(&edges, &entries);
    println!("The top 10 games are:");
    for (score,game1,game2) in top_games {
        println!("The similarity score was {:?}, with their game_ids being {} and {} and having a total of {} goals scored.",score,game1[0].game_id,game2[0].game_id,game1[0].total_goals)
    }
    let (max_node,max_centrality)=max_degree_centrality(graph);
    println!("The node with the highest degree centrality is {} with a centrality of {}. The assocaited game id is {}",max_node,max_centrality,entries[&(max_node as u64)][0].game_id)
}