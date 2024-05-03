#[cfg(test)]
use crate::GameEntry;
#[cfg(test)]
use crate::similarity;
#[cfg(test)]
use crate::Graph;
#[cfg(test)]
use crate::max_degree_centrality;
#[cfg(test)]
use crate::max_closeness_centrality;
#[cfg(test)]
use std::collections::HashMap;
fn approx_eq(a: f64, b: f64, tolerance: f64) -> bool {
    (a - b).abs() < tolerance
}
#[test]
fn test_calculate_similarity() {
    // Test 1: Check if the function correctly calculates the similarity score
    let game_entries1 = vec![
        GameEntry {
            node_id: 1,
            game_id: 1,
            minute: 10.0,
            description: "Right-footed shot".to_string(),
            home_club_goals: 1,
            away_club_goals: 0,
            total_goals: 1,
        },
        GameEntry {
            node_id: 1,
            game_id: 1,
            minute: 20.0,
            description: "Substitution".to_string(),
            home_club_goals: 1,
            away_club_goals: 0,
            total_goals: 1,
        },
    ];
    let game_entries2 = vec![
        GameEntry {
            node_id: 2,
            game_id: 2,
            minute: 10.0,
            description: "Substitution".to_string(),
            home_club_goals: 0,
            away_club_goals: 0,
            total_goals: 0,
        },
        GameEntry {
            node_id: 2,
            game_id: 2,
            minute: 24.0,
            description: "Substitution".to_string(),
            home_club_goals: 0,
            away_club_goals: 0,
            total_goals: 0,
        },
    ];
    let similarity_score = similarity::calculate_similarity(game_entries1, game_entries2);
    assert!(approx_eq(similarity_score, 0.590, 0.001));
}
#[test]
fn test_closeness_centrality() {
    // Test 1: Check if the function correctly calculates the closeness centrality
    let mut graph = Graph::new(5);
    graph.add_edge(1.0,0, 1);
    graph.add_edge(1.0,0, 2);
    graph.add_edge(1.0, 1, 2);
    graph.add_edge(1.0, 1, 3);
    graph.add_edge(1.0, 2, 3);
    graph.add_edge(1.0, 2, 4);
    graph.add_edge(1.0, 3, 4);

    let closeness_centralities = graph.closeness_centrality();
    let mut expected_closeness_centralities = HashMap::new();
    expected_closeness_centralities.insert(0, 1.0/(6.0/4.0));
    expected_closeness_centralities.insert(1, 1.0/(5.0/4.0));
    expected_closeness_centralities.insert(2, 1.0/(4.0/4.0));
    expected_closeness_centralities.insert(3, 1.0/(5.0/4.0));
    expected_closeness_centralities.insert(4, 1.0/(6.0/4.0));

    for (node, centrality) in closeness_centralities {
        assert_eq!(centrality, expected_closeness_centralities[&node]);
    }
    assert_eq!(max_closeness_centrality(&graph).0,2);
    assert_eq!(max_closeness_centrality(&graph).1,1.0);
}
#[test]
fn test_degree_centrality() {
    let mut graph = Graph::new(5);
    graph.add_edge(1.0, 0, 1);
    graph.add_edge(1.0, 0, 2);
    graph.add_edge(1.0, 1, 2);
    graph.add_edge(1.0, 1, 3);
    graph.add_edge(1.0, 2, 3);
    graph.add_edge(1.0, 2, 4);
    graph.add_edge(1.0, 3, 4);

    let (max_node_degree, max_centrality) = max_degree_centrality(&graph);
    assert_eq!(max_node_degree, 2);
    assert_eq!(max_centrality, 3.0);
}