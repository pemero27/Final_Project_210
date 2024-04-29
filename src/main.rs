use std::fs::File;
use std::io::BufRead;
use std::str::FromStr;
use std::collections::HashMap;
#[derive(Debug)]
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
    }
    result
}
fn calculate_similarity(entry1:Vec<GameEntry>,entry2:Vec<GameEntry>) {
    for event in entry1 {
        for event2 in entry2 {
            let similarity_score = 0.0;
        }
    }
}

//score metrics:
    // distance formula between minutes 
    // number of goals 
    // every event that is the same is a boost
    // number of subs??

fn main() {
   let entries=read_data("C:/Users/pje41/Final_Project_210/soccer-data/processed_data/game_data.csv");
   let mut entries_iter = entries.iter();
   while let Some((game_id1, game_entries1)) = entries_iter.next() {
       if let Some((game_id2, game_entries2)) = entries_iter.next() {
        //    let similarity_score = calculate_similarity(game_entries1, game_entries2);
        //    println!("Similarity score between game {} and game {}: {}", game_id1, game_id2, similarity_score);
       }
   }
    // Check one game_id against all other game_ids 
    //with each event being compared to each one and
    // an average score of the comparison being the simialrity for that event
}