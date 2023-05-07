//this module contians all the tests for the functions that need to be run separetly 

mod graph;
use crate::graph::{read_file, renumber_nodes, Graph};

mod features;
use crate::features::{find_last_feature_number,combine_data_vectors,read_csv_file};

mod filter;
use crate::filter::{eligible_nodes, highest_degree, highest_pct, highest_dummy, lowest_dummy};

use std::fs::File;
use std::io::{BufWriter, Write};

fn main() -> Result<(), Box<dyn std::error::Error>>{
    //create the test grpah
    let test_nodes: &[(usize, usize)] = &[
    (0, 3),
    (0, 5),
    (0, 8),
    (0, 11),
    (4, 3),
    (4, 5),
    (4, 9),
    (7, 3),
    (7, 8),
    (7, 12),
    (11, 6),
];
    write_graph_to_file(test_nodes);

    //use are functions to put the graph text into the Graph struct
    let nodes_test = read_file("graph.txt"); 
    let test_edges = renumber_nodes(&nodes_test);
    let test_graph = Graph::create(test_edges.len(), &test_edges);
    //thats how they look with renumber nodes
    //can't easily test all functions but can assume they are right given the other tests which require them to be right
    assert_eq!(test_edges,[(0, 1), (0, 2), (0, 3), (0, 4), (4, 9), (5, 1), (5, 2), (5, 6), (7, 1), (7, 3), (7, 8)] );

    //need these variables to test the main function eligible_nodes and highest_degree and highest_pct
    let test_outedges = &test_graph.outedges[..];
    let feature_data = get_feature_data_test()?;
    let degree = 2; //just to see different degrees and percenatges
    let pct = 0.2; //just to see different degrees and percenatges
    
    //test eligible_nodes
    let test_eligible_nodes = eligible_nodes(&test_outedges, &feature_data, degree, pct);
    assert_eq!(test_eligible_nodes,[(0, 4, 0.5, 2), (1, 3, 0.6666667, 2), (2, 2, 0.5, 1), (3, 2, 0.5, 1), (4, 2, 0.5, 1), (5, 3, 0.33333334, 1), (7, 3, 0.6666667, 2)]);

    //test highest_degree
    let by_degree = highest_degree(test_eligible_nodes.clone());
    assert_eq!(by_degree,[(0, 4, 0.5, 2), (1, 3, 0.6666667, 2), (5, 3, 0.33333334, 1), (7, 3, 0.6666667, 2), (2, 2, 0.5, 1), (3, 2, 0.5, 1), (4, 2, 0.5, 1)]);
    
    //test highest_pct
    let by_pct = highest_pct(test_eligible_nodes.clone());
    assert_eq!(by_pct,[(1, 3, 0.6666667, 2), (7, 3, 0.6666667, 2), (0, 4, 0.5, 2), (2, 2, 0.5, 1), (3, 2, 0.5, 1), (4, 2, 0.5, 1), (5, 3, 0.33333334, 1)]);

    // test highest_dummy
    let by_highest_dummy = highest_dummy(test_eligible_nodes.clone());
    assert_eq!(by_highest_dummy,[(0, 4, 0.5, 2), (1, 3, 0.6666667, 2), (7, 3, 0.6666667, 2), (2, 2, 0.5, 1), (3, 2, 0.5, 1), (4, 2, 0.5, 1), (5, 3, 0.33333334, 1)]);

    // test lowest dummy
    let by_lowest_dummy = lowest_dummy(test_eligible_nodes.clone());
    assert_eq!(by_lowest_dummy,[(2, 2, 0.5, 1), (3, 2, 0.5, 1), (4, 2, 0.5, 1), (5, 3, 0.33333334, 1), (0, 4, 0.5, 2), (1, 3, 0.6666667, 2), (7, 3, 0.6666667, 2)]);
    Ok(())
}

//all similar to other functions just specific to tests and creating all the test data required

fn write_graph_to_file(nodes: &[(usize, usize)]) -> String {
    let filename = String::from("graph.txt");
    let file = File::create(&filename).expect("Failed to create file");
    let mut writer = BufWriter::new(file);

    for (u, v) in nodes.iter() {
        writeln!(&mut writer, "{} {}", u, v).expect("Failed to write to file");
    }

    filename
}

fn featnames_test(filename: &str) {
    let featnames = vec![
        (0, "gender;anonymized feature 77"),
        (1, "gender;anonymized feature 78"),
    ];

    let file = File::create(filename).unwrap();
    let mut writer = BufWriter::new(file);

    for (idx, featname) in featnames.iter() {
        writeln!(&mut writer, "{} {}", idx, featname).unwrap();
    }
}

//same as the get_feature_data but the data is now the test data rest of function is same
fn get_feature_data_test() -> Result<Vec<(usize, usize)>, Box<dyn std::error::Error>> {
    featnames_test("featnames_test");
    write_test_feat("feat_test");
    //read that file
    let file_data = [
        ("featnames_test", "feat_test")];

    let mut vecs = Vec::new();

    for (featnames, feat) in file_data {
        let last_feature_number = find_last_feature_number(featnames, "gender;anonymized")?;
        let data = read_csv_file(feat, last_feature_number)?;
        vecs.push(data);
    }

    let combined_data = combine_data_vectors(vecs);

    Ok(combined_data)
}

fn write_test_feat(filename: &str) {
    let data = vec![
        (0, 0, 1),
        (1, 1, 0),
        (2, 0, 1),
        (3, 1, 0),
        (4, 0, 1),
        (5, 1, 0),
        (6, 0, 1),
        (7, 1, 0),
        (8, 0, 1),
        (9, 1, 0),
    ];

    let file = File::create(filename).expect("Unable to create file");
    let mut writer = BufWriter::new(file);

    for (a, b, c) in data {
        writeln!(writer, "{} {} {}", a, b, c).expect("Unable to write to file");
    }
}


