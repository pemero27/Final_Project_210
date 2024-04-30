use std::fs::File;
use std::io::BufRead;
use std::str::FromStr;
use std::collections::HashMap;
#[derive(Debug,Clone)]
struct GameEntry {
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
    for line in buf_reader {
        let line_str = line.expect("Error Reading");
        let events: Vec<String> = line_str.split(",").map(|s| s.to_string()).collect();
        // println!("{:?},{:?}",events[1],events[0]);
        let game_entry = GameEntry {
            game_id: u64::from_str(&events[0]).unwrap(),
            minute: f64::from_str(&events[1]).unwrap(),
            description: events[5].clone(),
            home_club_goals: u64::from_str(&events[13]).unwrap(),
            away_club_goals: u64::from_str(&events[14]).unwrap(),
            total_goals: u64::from_str(&events[13]).unwrap() + u64::from_str(&events[14]).unwrap()
        };
        result.entry(game_entry.game_id).or_insert(Vec::new()).push(game_entry);
        if result.len() == 500 {
            break
        }
    }
    result
}
fn calculate_similarity(entry1:Vec<GameEntry>,entry2:Vec<GameEntry>) -> f64{
    let mut sim_average =0.0;
    let mut total = 0.0;
    for event in &entry1 {
        for event2 in &entry2 {
            total += 1.0;
            let weights = vec![
            (0.5, event.minute, event2.minute),
            (1.0, event.home_club_goals as f64, event2.home_club_goals as f64),
            (1.0, event.away_club_goals as f64, event2.away_club_goals as f64),
            (3.0, event.total_goals as f64, event2.total_goals as f64),
            ];
            let mut weighted_sum = 0.0;
            let mut weight_sum = 0.0;

            for (weight, value1, value2) in weights {
                weighted_sum += weight * (value1 - value2).abs();
               weight_sum += weight;
            }
            if event.description==event2.description && event.description != "" {
                weighted_sum -= 5.0;
            }
            weighted_sum += 5.0;
            weight_sum += 5.0;
            sim_average += weighted_sum / weight_sum;
        }
    }
    sim_average = sim_average /total;
    sim_average
}
fn make_edge_list(entry_map:HashMap<u64,Vec<GameEntry>>) -> Vec<(f64, u64, u64)> {
    let mut edges: Vec<(f64,u64,u64)>=vec![];
    for (game_id1, game_entries1) in &entry_map {
        for (game_id2, game_entries2) in &entry_map {
            if game_id1!= game_id2 {
                let similarity_score = calculate_similarity(game_entries1.to_vec(), game_entries2.to_vec());
                edges.push((similarity_score, *game_id1, *game_id2));
            }
        }
    }
    edges
}
fn create_similarity_matrix(vertices:usize,edges:Vec<(f64,u64,u64)>) {
    let mut graph_matrix = vec![vec![false;vertices];vertices];
    for (similarity, game1,game2) in edges.iter() {
        graph_matrix[*game1 as usize][*game2 as usize] = true;
        graph_matrix[*game2 as usize][*game1 as usize] = true; 
    };
    for row in &graph_matrix {
        for entry in row.iter() {
            print!(" {} ",if *entry {"1"} else {"0"});
        }
        println!("");
    };
}
fn main() {
   let entries=read_data("C:/Users/pje41/Final_Project_210/soccer-data/processed_data/game_data.csv");
   let vertices = entries.len();
   let edges: Vec<(f64,u64,u64)> = make_edge_list(entries);
   create_similarity_matrix(vertices, edges)
    // Check one game_id against all other game_ids 
    //with each event being compared to each one and
    // an average score of the comparison being the simialrity for that event
}