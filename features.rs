//this module reads through and cleans the feature data(specifically the gender one but can be changed) for each node

use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

//read all the feature data given and extract the variable that we want using an index
pub fn read_csv_file(filepath: &str, var_index: usize) -> Result<Vec<(usize, usize)>, Box<dyn Error>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    let mut vec = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let fields: Vec<&str> = line.split_whitespace().collect();

        let node_id = fields[0].parse::<usize>()?;
        let dummy_var = fields[var_index].parse::<usize>()?;

        vec.push((node_id, dummy_var));
    }

    Ok(vec)
}

//gives us the variable index we require for the binary variable we are looking for
pub fn find_last_feature_number(filepath: &str, target: &str) -> Result<usize, Box<dyn Error>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    let mut last_feature_number = 0;

    for line in reader.lines() {
        let line = line?;
        let fields: Vec<&str> = line.split_whitespace().collect();

        if fields[1].starts_with(target) {
            let feature_number = fields[0].parse::<usize>().unwrap();
            if feature_number > last_feature_number {
                last_feature_number = feature_number;
            }
        }
    }

    Ok(last_feature_number)
}

//since the data came separated this combines it and also removes any overlaps
pub fn combine_data_vectors(vecs: Vec<Vec<(usize, usize)>>) -> Vec<(usize, usize)> {
    let mut combined_vec = Vec::new();

    for vec in vecs {
        for (node_id, dummy_var) in vec {
            if !combined_vec.iter().any(|&(id, _)| id == node_id) {
                combined_vec.push((node_id, dummy_var));
            }
        }
    }

    combined_vec
}

//this is how we get all the data given and read it using the above functions to help us extract just what we want
pub fn get_feature_data() -> Result<Vec<(usize, usize)>, Box<dyn std::error::Error>> {
    let file_data = [
        ("facebook/0.featnames", "facebook/0.feat"),
        ("facebook/107.featnames", "facebook/107.feat"),
        ("facebook/348.featnames", "facebook/348.feat"),
        ("facebook/414.featnames", "facebook/414.feat"),
        ("facebook/686.featnames", "facebook/686.feat"),
        ("facebook/698.featnames", "facebook/698.feat"),
        ("facebook/1684.featnames", "facebook/1684.feat"),
        ("facebook/1912.featnames", "facebook/1912.feat"),
        ("facebook/3437.featnames", "facebook/3437.feat"),
        ("facebook/3980.featnames", "facebook/3980.feat"),
    ];

    let mut vecs = Vec::new();

    for (featnames, feat) in file_data {
        let last_feature_number = find_last_feature_number(featnames, "gender;anonymized")?;
        let data = read_csv_file(feat, last_feature_number)?;
        vecs.push(data);
    }

    let combined_data = combine_data_vectors(vecs);

    Ok(combined_data)
}


#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    let combined_data = get_feature_data()?;
    //if you were curious
    for (node_id, dummy_var) in combined_data {
        println!("Node ID: {}, Dummy Var: {}", node_id, dummy_var);
    }

    Ok(())
}