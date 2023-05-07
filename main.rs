mod graph;
use crate::graph::{read_file, renumber_nodes, Graph};

mod features;
use crate::features::get_feature_data;

mod filter;
use crate::filter::{eligible_nodes, highest_degree, highest_pct, highest_dummy, lowest_dummy};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //create Graph struct using data
    let nodes_read = read_file("facebook_edges.txt");
    let fb_edges = renumber_nodes(&nodes_read);
    let fb_graph = Graph::create(fb_edges.len(), &fb_edges);

    //parameters needed for the elible nodes function
    //this gets the undirected adjacency list
    let fb_outedges = &fb_graph.outedges[..];
    //find the dummy variable associated with each node
    let feature_data = get_feature_data()?;
    //change to your desire
    let degree = 29;
    let pct = 0.6;

    let test_eligible_nodes = eligible_nodes(&fb_outedges, &feature_data, degree, pct); // define test_eligible_nodes
    println!("The nodes that have a minimum degree of {} and a minimum percenatge of {} of gender;anonymized 1:\n {:?}", degree, pct, test_eligible_nodes);

    let by_degree = highest_degree(test_eligible_nodes.clone());
    println!("The nodes that have a minimum degree of {} and a minimum percenatge of {} of gender;anonymized 1 ordered by DEGREE:\n{:?}", degree, pct, by_degree);

    let by_pct = highest_pct(test_eligible_nodes.clone());
    println!("The nodes that have a minimum degree of {} and a minimum percenatge of {} of gender;anonymized 1 ordered by PERCENTAGE:\n{:?}", degree, pct, by_pct);

    let by_highest_dummy = highest_dummy(test_eligible_nodes.clone());
    println!("The nodes that have a minimum degree of {} and a minimum percenatge of {} of gender;anonymized 1 ordered by HIGHEST DUMMEY:\n{:?}", degree, pct, by_highest_dummy);

    let by_lowest_dummy = lowest_dummy(test_eligible_nodes.clone());
    println!("The nodes that have a minimum degree of {} and a minimum percenatge of {} of gender;anonymized 1 ordered by LOWEST DUMMEY:\n{:?}", degree, pct, by_lowest_dummy);

    Ok(())
}



