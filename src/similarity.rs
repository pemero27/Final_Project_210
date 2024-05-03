use crate::GameEntry;
pub fn calculate_similarity(entry1:Vec<GameEntry>,entry2:Vec<GameEntry>) -> f64{
    let mut sim_sum = 0.0;
    let mut weight_sum = 0.0;
    let mut total_home_goals_diff = 0.0;
    let mut total_away_goals_diff = 0.0;
    let mut total_goals_diff = 0.0;
    let mut multiplier =0.0;
    let mut total_goals_checked = false;
    let mut home_goals_checked = false;
    let mut away_goals_checked = false;
    for event in &entry1 {
        for event2 in &entry2 {
            let weights = vec![
                (1.0, event.home_club_goals as f64, event2.home_club_goals as f64),
                (1.0, event.away_club_goals as f64, event2.away_club_goals as f64),
                (3.0, event.total_goals as f64, event2.total_goals as f64),
                (1.0, event.minute, event2.minute), 
            ];
            let mut weighted_sum = 0.0;
            for (weight, value1, value2) in &weights {
                if *weight == 3.0 && !total_goals_checked {
                    total_goals_diff += (*value1 - *value2).abs();
                    total_goals_checked = true;
                } else if *weight == 1.0 && !home_goals_checked {
                    total_home_goals_diff += (*value1 - *value2).abs();
                    home_goals_checked = true;
                } else if *weight == 1.0 && !away_goals_checked {
                    total_away_goals_diff += (*value1 - *value2).abs();
                    away_goals_checked = true;
                } else {
                    weighted_sum += *weight * (value1 - value2).abs();
                }
                weight_sum += *weight;
            }
            if event.description == event2.description && event.description != "Substitution" {
                multiplier+=0.25;
            }
            sim_sum += weighted_sum;
        }
    }
    let mut similarity = sim_sum / weight_sum;
    if multiplier != 0.0 {
        similarity = multiplier * (similarity*similarity).log10();
    }
    else {
        similarity = similarity.log10()* 3.0;
    }
    let normalized_similarity = 1.0 / (1.0 + similarity.abs());
    let total_diff = total_home_goals_diff + total_away_goals_diff + total_goals_diff;
    let adjusted_similarity = normalized_similarity - (total_diff * 0.005);
    adjusted_similarity 
}