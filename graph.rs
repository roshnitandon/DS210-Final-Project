//this module handles creating  agraph struct from the data and the functions that go with it

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

type Vertex = usize;
type ListOfEdges = Vec<(Vertex,Vertex)>;
type AdjacencyLists = Vec<Vec<Vertex>>;

//function to read facebook_edges.txt, very similar to hw9
pub fn read_file(path: &str) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = Vec::new();
    let file = File::open(path).expect("Could not open file");
    let buf_reader = BufReader::new(file).lines();
    for line in buf_reader {
        let line_str = line.expect("Error reading");
        let v: Vec<&str> = line_str.trim().split(' ').collect();
        let node_1: usize = v[0].parse::<usize>().unwrap();
        let node_2: usize = v[1].parse::<usize>().unwrap();
        result.push((node_1, node_2));
    }
    return result;
}

//make sure nodes go from 0-last node with no gaps in between
pub fn renumber_nodes(edges: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut unique_nodes: HashMap<usize, usize> = HashMap::new();
    let mut next_node_id = 0;
    let mut new_edges = Vec::new();
    let mut node_order = Vec::new();

    for &(src, dst) in edges.iter() {
        let new_src = match unique_nodes.get(&src) {
            Some(&id) => id,
            None => {
                let id = next_node_id;
                next_node_id += 1;
                unique_nodes.insert(src, id);
                node_order.push(id);
                id
            }
        };
        let new_dst = match unique_nodes.get(&dst) {
            Some(&id) => id,
            None => {
                let id = next_node_id;
                next_node_id += 1;
                unique_nodes.insert(dst, id);
                node_order.push(id);
                id
            }
        };
        new_edges.push((new_src, new_dst));
    }

    // sort the edges by the new node ids
    new_edges.sort_by_key(|&(src, dst)| (node_order[src as usize], node_order[dst as usize]));

    new_edges
}

#[derive(Debug)]
#[allow(dead_code)] //issues with n
pub struct Graph {
    pub n: usize,
    pub outedges: AdjacencyLists,
}


impl Graph {
    pub fn add_edges(&mut self, edges: &ListOfEdges) {
        for (i, j) in edges {
            //creates undirected graph which this data is
            self.outedges[*i].push(*j);
            self.outedges[*j].push(*i);
        }
    }

    pub fn create(n: usize, edges: &ListOfEdges) -> Graph {
        let mut g = Graph {
            n,
            outedges: vec![vec![]; n],
        };
        g.add_edges(edges);
        //remove the empty entries
        g.outedges.retain(|outedge| !outedge.is_empty());
        g
    }

    //if you were curious on how many degrees each node has
    pub fn print_degrees(&self) {
        for (i, l) in self.outedges.iter().enumerate() {
            println!("Vertex {} has degree {}", i, l.len());
        }
    }
}

#[allow(dead_code)]
fn main() {
    let nodes_read = read_file("facebook_edges.txt");
    let fb_edges = renumber_nodes(&nodes_read);
    let fb_graph = Graph::create(fb_edges.len(), &fb_edges);
    fb_graph.print_degrees();
}