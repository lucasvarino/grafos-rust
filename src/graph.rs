use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

use crate::edge::Edge;
use crate::node::Node;

#[derive(Debug)]
pub struct Graph {
    adj_list: HashMap<usize, HashSet<Edge>>,
    node_list: HashMap<usize, Node>,
    pub order: i32,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            order: 0,
            node_list: HashMap::new(),
            adj_list: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, src: usize, dest: usize, weight: i32) {
        if weight < 1 {
            panic!("Peso da aresta deve ser maior que 0");
        }

        self.adj_list
            .entry(src)
            .or_insert_with(HashSet::new)
            .insert(Edge::new(dest, weight));

        self.adj_list
            .entry(dest)
            .or_insert_with(HashSet::new)
            .insert(Edge::new(src, weight));
        
        self.node_list
        .get_mut(&dest)
        .unwrap()
        .increment_degree();

        self.node_list
        .get_mut(&src)
        .unwrap()
        .increment_degree();
    }

    pub fn get_edge_weight(&mut self, src: i32, dest: i32) -> io::Result<i32> {
        let src_usize = src as usize;
        let dest_usize = dest as usize;
        let weight: i32 = self
            .adj_list
            .entry(src_usize)
            .or_default()
            .get(&dest_usize)
            .unwrap()
            .get_weight();
        Ok(weight)
    }

    pub fn remove_edge(&mut self, src: i32, dest: i32) -> io::Result<()> {
        let src_usize = src as usize;
        let dest_usize = dest as usize;
        // let weight: i32 = self.get_edge_weight(src, dest)?;

        let remove_src = self
            .adj_list
            .entry(src_usize)
            .or_default()
            .remove(&dest_usize);

        let remove_dest = self
            .adj_list
            .entry(dest_usize)
            .or_default()
            .remove(&src_usize);
        

        if remove_src && remove_dest {

            self.node_list
            .get_mut(&(dest as usize))
            .unwrap()
            .decrement_degree();

            self.node_list
            .get_mut(&(src as usize))
            .unwrap()
            .decrement_degree();

            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Uma ou ambas as arestas não foram encontradas",
            ))
        }
    }

    fn remove_all_edges(&mut self, node: i32) {
        for (_node, edges) in self.adj_list.iter_mut() {
            edges.remove(&(node as usize));
        }
    }

    pub fn add_node(&mut self, id: usize) {
        let weight: f32 = ((id % 200) + 1) as f32;

        if self.node_list.len() as i32 == self.order {
            return;
        }

        self.node_list
            .entry(id as usize)
            .or_insert(Node::new(id, weight));
    }

    pub fn get_num_edges(&self) -> usize{
        self.adj_list
        .values()
        .map(|edges| edges.len())
        .sum::<usize>() / 2
    }

    pub fn is_complete(&self) -> bool{
        let num_edges = self.get_num_edges();
        let edge_condition: bool = num_edges == (self.order*(self.order-1)/2) as usize;

        // let degree_condition: bool = self.adj_list.iter().all(|(_node, edges)| edges.len() == (self.order-1) as usize);
        let degree_condition: bool = self.node_list.iter().all(|(_node, node)| node.get_degree() == (self.order-1) as u32);
        
        edge_condition && degree_condition
    }

    pub fn get_union(&self, other : Graph)->Graph{
        let mut union = Graph::new();

        if self.order > other.order {
            union.order = self.order;
        }else{
            union.order = other.order;
        }

        for i in 1..(union.order+1) {
            union.add_node(i as usize);
        }
        

        self.adj_list
        .iter()
        .map(|(node, edges)| {
            for edge in edges {
                union.add_edge(*node, edge.get_dest(), edge.get_weight());
            }
        });

        other.adj_list
        .iter()
        .map(|(node, edges)| {
            for edge in edges {
                union.add_edge(*node, edge.get_dest(), edge.get_weight());
            }
        });

        union
    }

    pub fn get_complement(&self) -> io::Result<Graph>{
        if self.is_complete() {
           return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "O grafo já é completo",
            ));
        }
        let mut complement = Graph::new();
        complement.order = self.order;

        complement.node_list = self.node_list.clone();

        match self.adj_list
        .iter()
        .next() 
        {
            Some((node, edges)) => {
                for i in 1..(self.order+1) {
                    if i != *node as i32 && !edges.contains(&(i as usize)) {
                        complement.add_edge(*node, i as usize, 1);
                    }
                }
            }
            None => (),
        }

        Ok(complement)
    }

    pub fn remove_node(&mut self, node: i32) {
        let node_usize = node as usize;

        match self.adj_list.remove(&node_usize) {
            Some(_) => {
                self.remove_all_edges(node);
                self.node_list.remove(&node_usize);
            }
            None => (),
        }
    }

    pub fn get_open_neighborhood(&mut self, node: i32) -> &HashSet<Edge> {
        self.adj_list.entry(node as usize).or_default()
    }

    pub fn get_closed_neighborhood(&mut self, node: i32) -> HashSet<Edge> {
        self.adj_list
            .iter()
            .filter(|(_index, edge)| edge.contains(&(node as usize)))
            .flat_map(|(_index, edge)| edge)
            .cloned()
            .collect()
    }

    pub fn print_graph(&self) {
        for (node, edges) in &self.adj_list {
            println!("Adjacency list of node {}:", node);
            print!("head");

            for e in edges {
                print!(" -> {}", e.get_dest());
            }

            println!();
        }
    }
}

pub fn read_graph_from_file<P>(filename: P) -> Result<Graph, io::Error>
where
    P: AsRef<Path>,
{
    let mut graph = Graph::new();

    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();

        if index > 0 && parts[0] == "e" {
            let src: usize = parts[1].parse().unwrap();
            let dst: usize = parts[2].parse().unwrap();
            let weight: i32 = parts[3].parse().unwrap();

            graph.add_node(src);
            graph.add_node(dst);

            graph.add_edge(src, dst, weight);
            
        } else {
            graph.order = parts[0].parse().unwrap();
        }
    }

    Ok(graph)
}
