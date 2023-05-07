//this module handles some of the most important functions that help the user filter thriugh the different nodes

//fucntion that finds all the eligble nodes given minimum degree and minimum dummy variable percentage
pub fn eligible_nodes(adj_list: &[Vec<usize>], dummy_list: &Vec<(usize, usize)>, min_degree: usize, min_dummy_pct: f32) -> Vec<(usize, usize, f32, usize)> {
    let num_nodes = adj_list.len();
    let mut node_degrees = vec![0; num_nodes];
    let mut node_dummy_counts = vec![0; num_nodes];
    let mut node_dummy_counts_ones = vec![0; num_nodes];
    
    // count degrees and dummy variable occurrences
    for (node, adj_nodes) in adj_list.iter().enumerate() {
        node_degrees[node] = adj_nodes.len();
        for adj_node in adj_nodes {
            if let Some(dummy) = dummy_list.get(*adj_node) {
                node_dummy_counts[node] += dummy.1;
                node_dummy_counts_ones[node] += if dummy.1 == 1 { 1 } else { 0 };
            }
        }
    }

    // filter nodes based on degree and dummy variable percentage
    let mut result = Vec::new();
    for node in 0..num_nodes {
        let degree = node_degrees[node];
        let _dummy_count = node_dummy_counts[node];
        let dummy_count_ones = node_dummy_counts_ones[node];
        let dummy_pct_ones = if degree == 0 { 0.0 } else { dummy_count_ones as f32 / degree as f32 };
        if degree >= min_degree && dummy_pct_ones >= min_dummy_pct {
            result.push((node, degree, dummy_pct_ones, dummy_count_ones));
        }
    }
    result
}


#[allow(dead_code)]
pub fn highest_pct(tuples: Vec<(usize, usize, f32, usize)>) -> Vec<(usize, usize, f32, usize)> {
    let mut sorted_tuples = tuples;
    sorted_tuples.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
    sorted_tuples
}

#[allow(dead_code)]
pub fn highest_degree(tuples: Vec<(usize, usize, f32, usize)>) -> Vec<(usize, usize, f32, usize)> {
    let mut sorted_tuples = tuples;
    sorted_tuples.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    sorted_tuples
}

pub fn highest_dummy(tuples: Vec<(usize, usize, f32, usize)>) -> Vec<(usize, usize, f32, usize)> {
    let mut sorted_tuples = tuples;
    sorted_tuples.sort_by(|a, b| b.3.partial_cmp(&a.3).unwrap());
    sorted_tuples
}

pub fn lowest_dummy(tuples: Vec<(usize, usize, f32, usize)>) -> Vec<(usize, usize, f32, usize)> {
    let mut sorted_tuples = tuples;
    sorted_tuples.sort_by(|a, b| a.3.partial_cmp(&b.3).unwrap());
    sorted_tuples
}

#[allow(dead_code)]
fn main() {

}